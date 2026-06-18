# LazyDJ Architecture

Versión: 1.0

---

# 1. Principios Arquitectónicos

El sistema deberá seguir los siguientes principios:

## Audio First

El audio es el componente más crítico.

Ningún componente podrá bloquear:

* Audio Engine
* DSP Engine
* Mixer Engine

La interfaz gráfica, base de datos y tareas externas deberán ejecutarse de forma desacoplada.

---

## Modularidad

Cada módulo deberá tener una única responsabilidad.

Se prohíben dependencias circulares.

---

## Event Driven

Los módulos se comunicarán mediante eventos.

No deberán invocarse directamente entre sí salvo en casos excepcionales.

---

## Extensibilidad

Todo componente importante deberá permitir:

* Plugins
* Nuevos dispositivos
* Nuevos formatos
* Nuevos efectos

---

# 2. Arquitectura General

```
                ┌─────────────┐
                │ GUI / UX    │
                └──────┬──────┘
                       │
                       ▼

                ┌─────────────┐
                │ Event Bus   │
                └──────┬──────┘

  ┌──────────────┬─────┴─────┬──────────────┐
  ▼              ▼           ▼              ▼
```

Audio Engine   Library     Hardware      Downloader

```
  ▼              ▼           ▼
  └──────┬───────┴───────────┘
         ▼

     DSP Engine

         ▼

   Audio Output
```

---

# 3. Workspace Layout

lazydj/

├── apps/
│
│   └── lazydj-desktop
│
├── crates/
│
│   ├── core
│   ├── audio
│   ├── mixer
│   ├── dsp
│   ├── bpm
│   ├── key_detection
│   ├── waveform
│   ├── stems
│   ├── hardware
│   ├── library
│   ├── downloader
│   ├── plugins
│   ├── database
│   ├── cache
│   ├── events
│   └── ui
│
├── plugins/
│
├── assets/
│
├── docs/
│
└── tests/

---

# 4. Thread Model

Se utilizará una arquitectura multihilo.

## Audio Thread

Prioridad máxima.

Responsabilidades:

* Playback
* Mixing
* DSP
* Output

Nunca debe bloquearse.

Nunca debe acceder a SQLite.

Nunca debe realizar IO.

---

## UI Thread

Responsabilidades:

* Render
* Animaciones
* Eventos de usuario

Puede fallar sin afectar audio.

---

## Background Workers

Responsabilidades:

* BPM Analysis
* Stems
* Descargas
* Escaneo de biblioteca

Utilizar:

* Tokio
* Rayon

---

## Database Thread

Responsabilidades:

* Escritura SQLite
* Caché

Nunca interactúa directamente con audio.

---

# 5. Event Bus

Todo el sistema utilizará eventos.

Ejemplo:

User Action
│
▼

Event Bus
│
▼

Audio Engine

---

## Eventos Globales

TrackLoaded

DeckPlay

DeckPause

DeckStop

CueAdded

LoopCreated

BpmDetected

KeyDetected

DeviceConnected

DeviceDisconnected

StemGenerated

TrackDownloaded

---

# 6. Audio Pipeline

Track
│
▼

Decoder

│
▼

Time Stretch

│
▼

Pitch Shift

│
▼

EQ

│
▼

Effects

│
▼

Mixer

│
▼

Limiter

│
▼

Audio Device

---

# 7. Deck Architecture

Cada deck es independiente.

struct Deck

* Track
* BPM
* Pitch
* Gain
* EQ
* Filters
* Cue Points
* Loop State

---

Deck A y Deck B jamás comparten estado mutable.

La sincronización ocurre mediante eventos.

---

# 8. Hardware Abstraction Layer

Objetivo:

Soportar múltiples controladoras sin modificar el núcleo.

---

Hardware Layer

├── MIDI Driver
├── HID Driver
├── Device Profiles
└── Mapping Engine

---

# 9. Device Profiles

Cada dispositivo tendrá un perfil.

profiles/

├── ddj400.toml
├── flx4.toml
├── inpulse500.toml
└── mixtrack.toml

---

Ejemplo:

play_button = deck_a.play

cue_button = deck_a.cue

crossfader = mixer.crossfader

---

# 10. Animation Engine

Las animaciones nunca deberán depender del audio thread.

---

Animation Loop

60 FPS

Render State

* platter_rotation
* waveform_position
* vu_meters
* cue_markers

---

La UI obtiene snapshots del estado.

Nunca accede al estado real del audio.

---

# 11. Waveform System

Se generan durante la indexación.

No se generan en tiempo real.

---

Almacenamiento:

waveforms/

track_id.wave

---

Datos almacenados:

* peaks
* rms
* beatgrid

---

# 12. BPM Analysis

Pipeline:

Decode
↓
Mono Conversion
↓
FFT
↓
Transient Detection
↓
Beat Tracking
↓
Store Result

---

# 13. Key Detection

Pipeline:

Decode
↓
Chromagram
↓
Krumhansl
↓
Camelot Conversion

---

# 14. Stems Engine

No se ejecuta en Audio Thread.

---

Workflow:

Track
↓
Worker Queue
↓
Demucs
↓
Cache
↓
Database

---

Salida:

stems/

track_id/

vocals.wav

drums.wav

bass.wav

other.wav

---

# 15. Downloader

Implementación:

yt-dlp

---

Workflow:

URL
↓
Validation
↓
Metadata
↓
Download
↓
Tag Extraction
↓
Library Import

---

# 16. Database

SQLite

Tablas:

tracks

playlists

history

waveforms

cue_points

device_profiles

stems

---

# 17. Cache Layer

cache/

bpm/

waveforms/

stems/

thumbnails/

analysis/

---

# 18. Plugin System

Objetivo:

Agregar funcionalidades sin recompilar.

---

Plugins soportados:

* Effects
* Visualizers
* AI Tools
* Hardware Drivers

---

Plugin Lifecycle

Load
↓
Register
↓
Initialize
↓
Execute
↓
Unload

---

# 19. Logging

Utilizar:

tracing

tracing-subscriber

---

Niveles:

ERROR

WARN

INFO

DEBUG

TRACE

---

# 20. Error Strategy

Todas las capas deberán utilizar:

Result<T>

Errores tipados.

Prohibido:

unwrap()

expect()

en producción.

---

# 21. Security

URLs descargadas deberán validarse.

Plugins ejecutados con permisos limitados.

Validación de formatos de audio.

Protección contra archivos corruptos.

---

# 22. Future Extensions

* Streaming Services
* Cloud Library
* Rekordbox Import
* AI AutoMix
* Voice Commands
* Network Sync
* Remote Controller App
* OBS Integration

---

# 23. Success Criteria

La arquitectura será considerada válida si:

* Audio funciona sin interrupciones.
* UI puede congelarse sin afectar reproducción.
* Nuevas controladoras pueden añadirse mediante perfiles.
* Nuevos efectos pueden añadirse mediante plugins.
* El sistema soporta crecimiento sin refactorización masiva.
* Latencia inferior a 10 ms.
