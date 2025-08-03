# Juego de Ray Casting - Edición Rust

Un motor de juego 3D completo construido en Rust utilizando algoritmos de ray casting, con múltiples niveles, efectos visuales, audio y una experiencia de juego completa.

## Descripción del Proyecto

Este proyecto implementa un motor de ray casting 3D desde cero en Rust, creando una experiencia de juego similar a los clásicos FPS de los años 90 como Wolfenstein 3D. El juego presenta un laberinto en 3D donde el jugador debe navegar a través de diferentes niveles, evitando zonas de peligro y encontrando la salida. El motor renderiza gráficos 3D en tiempo real utilizando técnicas de ray casting, manteniendo un rendimiento óptimo de 60 FPS.

## Video Demostración

<div align="center">
  <iframe width="560" height="315" src="https://youtu.be/6mf6UVw-2Os" 
          title="Demostración del Juego de Ray Casting" 
          frameborder="0" 
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
          allowfullscreen>
  </iframe>
  *Video de demostración del juego ejecutándose en tiempo real*
</div>



### Enlace
- [Ver en YouTube](https://youtu.be/6mf6UVw-2Os)

</div>

## Criterios de Evaluación Cumplidos
Este proyecto cumple con los siguientes requisitos de evaluación:

### ✅ **Criterio Estético (0-30 puntos)**
- Diseño visual atractivo con diferentes tipos de paredes y colores
- Efectos visuales inmersivos (linterna, daño)
- Interfaz de usuario limpia y funcional
- Animaciones suaves y transiciones fluidas

### ✅ **Rendimiento de 60 FPS (15 puntos)**
- **Mantiene aproximadamente 60 FPS constantes**
- **Los FPS se muestran en tiempo real en la consola**
- Motor optimizado con algoritmos eficientes de ray casting
- Compilación en modo release para máximo rendimiento

### ✅ **Efectos Visuales (15 puntos)**
- **Sistema de linterna (tecla F)**: Iluminación dinámica con cono de luz
- **Efectos de daño**: Sacudida de pantalla y tinte rojo al recibir daño

### ✅ **Cámara con Rotación de Mouse (20 puntos)**
- **Cámara 3D completamente implementada**
- **Rotación horizontal con el mouse** para control de cámara estilo FPS
- Controles suaves y responsivos
- Captura de mouse mejorada durante el juego

### ✅ **Minimapa (10 puntos)**
- **Minimapa en tiempo real** que muestra la posición del jugador
- **Indicador direccional** con flecha que muestra hacia dónde mira el jugador
- Representación visual del mundo y las paredes
- Actualización en tiempo real de la posición

### ✅ **Efectos de Sonido (10 puntos)**
- **Sonido de fondo**: Sonido de fondo, en el menu y en el juego
- **Efectos de sonido**: pasos, sonidos de éxito
- **Audio procedural** generado con ondas sinusoidales

### ✅ **Pantalla de Bienvenida (5 puntos)**
- Introducción visual atractiva al juego
- Transición suave al menú principal

### ✅ **Selección de Múltiples Niveles (10 puntos adicionales)**
- **Pantalla de selección de niveles** funcional
- **3 niveles diferentes** con diseños únicos:
  - Nivel 1: Laberinto para principiantes
  - Nivel 2: Fortaleza de piedra
  - Nivel 3: Laberinto de metal
- Navegación entre niveles con teclas numéricas (1, 2, 3)

### ✅ **Pantalla de Éxito (10 puntos)**
- **Pantalla de victoria** cuando se completa un nivel
- Celebración con estrellas
- Opciones para continuar o reiniciar
- Condición de victoria al llegar a la salida

### ✅ **Control de Vida del Jugador (5 puntos)**
- **Sistema de salud** con 100 HP iniciales
- **Barra de vida visual** en la interfaz
- **Zonas de peligro** (áreas naranjas) que causan mucho daño
- **Daño por colisión** con paredes
- Retroalimentación visual al recibir daño

**Puntuación Total Estimada: 120+ puntos**

## Características del Juego

### Jugabilidad Principal
- **Motor de Ray Casting 3D**: Renderizado 3D en tiempo real
- **Movimiento del Jugador**: Controles WASD con detección de colisiones
- **Vista con Mouse**: Rotación horizontal de cámara
- **Múltiples Niveles**: 3 niveles diferentes con diseños únicos

### Características Visuales
- **Diferentes Texturas de Paredes**:
  - Paredes de ladrillo rojo
  - Paredes de piedra azul
  - Paredes de madera verde
  - Paredes de metal amarillo
- **Efectos Visuales**:
  - Sistema de linterna (tecla F)
  - Efectos de daño con sacudida y tinte rojo

### Sistema de Audio
- **Sonido de Fondo**: Música de fondo, en el menú y en el juego
- **Efectos de Sonido**: Pasos, sonidos de éxito
- **Audio Procedural**: Audio generado con ondas sinusoidales

### Estados del Juego
- **Pantalla Splash**: Introducción animada
- **Selección de Nivel**: Elige entre 3 niveles diferentes
- **Gameplay**: Experiencia completa de ray casting 3D
- **Pantalla de Éxito**: Celebración de victoria
- **Pantalla de Game Over**: Pantalla de muerte con opciones de reinicio

## Controles

- **WASD / Flechas**: Mover jugador
- **Mouse**: Mirar alrededor (rotación horizontal)
- **F**: Alternar linterna
- **Espacio**: Continuar desde pantalla splash, reiniciar desde pantalla de éxito
- **1, 2, 3**: Seleccionar niveles en pantalla de selección
- **R**: Reiniciar nivel (pantalla de game over)
- **M**: Volver al menú (pantalla de game over)
- **Escape**: Salir del juego

## Descripción de Niveles

### Nivel 1 - Laberinto para Principiantes
Un laberinto simple con tipos de paredes mixtas, zonas de peligro y un punto de salida. Perfecto para aprender los controles.

### Nivel 2 - Fortaleza de Piedra
Una estructura tipo fortaleza con paredes de piedra azul, cámara central de peligro y diseño estratégico.

### Nivel 3 - Laberinto de Metal
Un laberinto complejo con paredes de metal amarillo, patrones alternados y un corredor central de peligro.

## Construcción y Ejecución

### Prerrequisitos
- Rust (versión estable más reciente)
- Administrador de paquetes Cargo

### Instrucciones de Construcción
```bash
# Clonar o navegar al directorio del proyecto
cd RaysCasting-Rust-Proy

# Construir el proyecto
cargo build

# Ejecutar el juego
cargo run --release
```

### Dependencias
- `minifb`: Gestión de ventanas y renderizado de buffer de píxeles
- `rodio`: Reproducción de audio y generación de sonido
- `rand`: Generación de números aleatorios para efectos
- `image`: Utilidades de procesamiento de imágenes

## Arquitectura

El juego está estructurado en varios módulos:

- **main.rs**: Punto de entrada y bucle principal del juego
- **game.rs**: Gestión de estados del juego y coordinación
- **player.rs**: Movimiento del jugador y detección de colisiones
- **map.rs**: Definiciones de niveles y datos del mapa
- **raycaster.rs**: Motor de renderizado 3D
- **audio.rs**: Sistema de sonido y generación de música
- **ui.rs**: Interfaz de usuario y renderizado de HUD
- **effects.rs**: Efectos visuales y post-procesamiento

## Rendimiento

El juego mantiene aproximadamente 60 FPS en hardware moderno con:
- Resolución 800x600
- Ray casting en tiempo real
- Múltiples efectos visuales
- Procesamiento de audio
- Renderizado de UI
