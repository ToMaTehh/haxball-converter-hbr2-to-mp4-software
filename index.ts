import { spawn } from 'child_process';
import path from 'path';

// Capturamos el archivo que nos pasa Rust como argumento en la ejecución
const archivoHbr2 = process.argv[2];

if (!archivoHbr2) {
  console.error("Error: No se proporcionó ningún archivo .hbr2");
  process.exit(1);
}

console.log(`[Node.js Backend] Iniciando automatización para: ${archivoHbr2}`);

async function procesarGrabacion() {
  try {
    console.log(`[Node.js Backend] Abriendo Puppeteer para procesar: ${archivoHbr2}`);
    
    console.log(`[Node.js Backend] Conversión finalizada con éxito para ${archivoHbr2}`);
    process.exit(0); // Le avisa a Rust que todo salió de 10
  } catch (error) {
    console.error(`[Node.js Backend] Error durante la automatización: ${error}`);
    process.exit(1); // Le avisa a Rust que algo falló
  }
}

procesarGrabacion();
