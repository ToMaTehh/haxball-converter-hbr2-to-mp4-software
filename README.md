# 💻 — HaxRec CONVERSOR .hbr2 > .mp4 (v0.1.0)

## 🖥️ — INTRODUCCIÓN
***HaxRec Converter** es una aplicación de escritorio diseñada para automatizar la conversión de archivos de repetición de HaxBall (`.hbr2`) a videos de alta calidad (`.mp4`). Olvidate de grabar la pantalla manualmente; la app se encarga de todo el proceso de renderizado en segundo plano.*

## 📄 — CARACTERÍSTICAS

*   <ins>**Interfaz Moderna y Limpia:**</ins> *Desarrollada con modo oscuro nativo usando **Svelte**.*
*   <ins>**Rendimiento Nativo:**</ins> *El núcleo de la aplicación está construido sobre **Tauri** y **Rust**, garantizando ligereza y velocidad.*
*   <ins>**Automatización con Node.js:**</ins> *Control de navegación inteligente mediante **Puppeteer** para procesar la repetición exactamente como en el juego.*
*   <ins>**Conversión a 60 FPS:**</ins> *Renderizado fluido listo para subir a YouTube o compartir con tu equipo.*

## 🛠️ — STACK ANALÓGICO

*La aplicación utiliza una arquitectura híbrida dividida en tres capas principales:*

| Capa | Tecnología | Función |
| :--- | :--- | :--- |
| **Frontend** | ```Svelte + TypeScript``` | *Interfaz de usuario y selector de archivos* |
| **Backend** | ```Rust (Tauri)``` | *Orquestación, manejo de procesos y seguridad* |
| **Automatización** | ```Node.js (Puppeteer + FFmpeg)``` | *Carga del replay y renderizado de video* |

## 📦 — INSTALACIÓN Y DESARROLLO

### Si querés clonar el repositorio y correr el proyecto en modo desarrollo, seguí estos pasos:

### 1. Prerrequisitos
*Asegurate de tener instalado [Node.js](https://nodejs.org/) y el entorno de [Rust](https://www.rust-lang.org/).*

### 2. Clonar el repositorio e instalar dependencias
```bash
git clone https://github.com/ToMaTehh/haxball-converter-hbr2-to-mp4-software
```
### 3. Instalar la dependencia de ejecución TSX
```npm install -D tsx```

### 4. Ejecutar la aplicación en modo desarrollo
```npm run tauri dev```

## 🔧 — ¿CÓMO FUNCIONA POR DENTRO?
- *El usuario selecciona un archivo .hbr2 desde la interfaz de Svelte.*
- *El frontend invoca un comando seguro hacia el backend de Rust.*
- *Rust levanta de forma nativa el script de automatización (index.ts) pasándole el archivo como argumento.*
- *Node.js interactúa con el juego en segundo plano y exporta el video final en alta definición.*
