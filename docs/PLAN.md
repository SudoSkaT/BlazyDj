# LazyDJ

Sistema de mezcla DJ y manipulación avanzada de audio desarrollado en Rust.

---

# 1. Visión del Proyecto

LazyDJ es una plataforma de mezcla musical profesional inspirada en VirtualDJ, Rekordbox, Serato y Ableton Live, diseñada para ser ligera, extensible y multiplataforma.

El objetivo es proporcionar una experiencia profesional de DJing y procesamiento de audio mediante una arquitectura modular basada en Rust.

El sistema deberá funcionar correctamente tanto con teclado y ratón como con controladores DJ externos.

---

# 2. Objetivos Principales

## Objetivos Funcionales

* Reproducir audio en tiempo real.
* Mezclar múltiples pistas simultáneamente.
* Detectar BPM automáticamente.
* Sincronización automática entre decks.
* Gestión de bibliotecas musicales.
* Descarga e importación de canciones desde URLs compatibles.
* Separación de stems.
* Procesamiento DSP.
* Integración con hardware DJ.
* Automatización de mezclas.
* Sistema de plugins.

## Objetivos Técnicos

* Baja latencia.
* Consumo eficiente de memoria.
* Arquitectura desacoplada.
* Alta extensibilidad.
* Compatibilidad Linux, Windows y macOS.

---

# 3. Casos de Uso

## DJ Principiante

* Importa canciones.
* Crea playlists.
* Utiliza AutoMix.

## DJ Profesional

* Utiliza múltiples decks.
* Controla hardware externo.
* Configura efectos avanzados.
* Trabaja con stems.

## Productor

* Extrae stems.
* Analiza BPM y tonalidad.
* Exporta proyectos.

---

# 4. Requisitos Funcionales

## RF-001 Gestión de Biblioteca

El sistema deberá:

* Importar carpetas.
* Escanear metadatos.
* Detectar BPM.
* Detectar tonalidad.
* Indexar canciones.

---

## RF-002 Reproducción

El sistema deberá:

* Play
* Pause
* Stop
* Seek
* Cue

---

## RF-003 Mezcla

El sistema deberá:

* Crossfader.
* Ganancia independiente.
* EQ por canal.
* Filtros.
* Sincronización.

---

## RF-004 BPM

El sistema deberá:

* Detectar BPM automáticamente.
* Permitir corrección manual.
* Guardar BPM en caché.

---

## RF-005 Tonalidad

El sistema deberá:

* Detectar key.
* Mostrar Camelot Wheel.
* Recomendar mezclas armónicas.

---

## RF-006 Loops

* Loop automático.
* Loop manual.
* Beat Loop.

---

## RF-007 Hot Cues

* Crear.
* Editar.
* Eliminar.
* Exportar.

---

## RF-008 Stems

Separar:

* Vocals
* Drums
* Bass
* Instruments

---

## RF-009 Descarga de Audio

Permitir:

* URLs de YouTube.
* Playlists.
* SoundCloud (futuro). 

---

## RF-010 Grabación

* Grabar sesión completa.
* Exportar WAV.
* Exportar FLAC.
* Exportar MP3.

---

# 5. Compatibilidad Hardware DJ

## Detección Automática

El sistema deberá detectar automáticamente:

* MIDI Controllers.
* HID Controllers.
* Audio Interfaces.

## Dispositivos Compatibles Inicialmente

* Pioneer DDJ Series.
* Pioneer FLX Series.
* Hercules Inpulse.
* Numark Mixtrack.
* Traktor Kontrol.

## Funcionalidades

* Mapeo automático.
* Mapeo manual.
* Perfiles personalizados.
* Hot-plug.

---

# 6. Interfaz de Usuario

## Principios

* Operación rápida.
* Información clara.
* Mínimo número de clics.
* Flujo de trabajo profesional.

---

## Vista Principal

┌───────────────────────────────────────────────┐
│ LIBRARY                                       │
├───────────────────────────────────────────────┤
│ DECK A                    DECK B              │
│                                               │
│      PLATO A              PLATO B             │
│                                               │
│     WAVEFORM GLOBAL                           │
│                                               │
│ FX PANEL                                      │
│                                               │
│ MIXER                                          │
├───────────────────────────────────────────────┤
│ STATUS BAR                                    │
└───────────────────────────────────────────────┘

---

## Platos Animados

Cada deck deberá mostrar:

* Disco girando.
* Velocidad real.
* Posición de reproducción.
* Estado del pitch.
* Indicador de scratching.

Animaciones:

* 60 FPS mínimo.
* Sin bloquear audio.
* Renderizado desacoplado.

---

## Waveforms

Mostrar:

* Forma de onda completa.
* Forma de onda ampliada.
* Beat markers.
* Cue markers.
* Loop markers.

---

## Mixer

Mostrar:

* Faders.
* Crossfader.
* Ganancia.
* EQ.
* Filtros.
* Medidores VU.

---

# 7. Arquitectura

Frontend
↓
Application Layer
↓
Audio Engine
↓
DSP Engine
↓
Hardware Layer

Módulos independientes:

* library
* mixer
* audio
* downloader
* stems
* database
* plugins
* ui

---

# 8. Requisitos No Funcionales

## RNF-001 Rendimiento

Latencia máxima:

< 10 ms

---

## RNF-002 Escalabilidad

Permitir:

* Nuevos efectos.
* Nuevos dispositivos.
* Nuevos formatos.

---

## RNF-003 Confiabilidad

Audio nunca debe interrumpirse por:

* Render UI.
* Descargas.
* Escaneo de biblioteca.

---

## RNF-004 Observabilidad

Logs estructurados.

Métricas:

* CPU
* Memoria
* Latencia
* FPS UI

---

# 9. Roadmap

Fase 1

* Biblioteca.
* Reproducción.
* Decks.
* Mixer.

Fase 2

* BPM.
* Key Detection.
* Loops.
* Hot Cues.

Fase 3

* Hardware DJ.
* MIDI.
* HID.

Fase 4

* Stems.
* IA.
* AutoMix.

Fase 5

* Plugins.
* Streaming.
* Cloud Sync.

---

# 10. Criterios de Éxito

El proyecto será considerado exitoso cuando:

* Permita mezclar dos pistas en tiempo real.
* Detecte BPM automáticamente.
* Controle hardware DJ externo.
* Mantenga baja latencia.
* Permita separación de stems.
* Sea extensible mediante plugins.
* Funcione de forma estable en Linux.
