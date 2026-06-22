<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"; // Esto comunica Svelte con Rust

  let status = "Esperando archivo...";
  let filePath = "";
  let fileInput: HTMLInputElement;

  function manejarSeleccion(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const archivo = target.files[0];
      filePath = archivo.name; 
      status = `Archivo "${archivo.name}" cargado listo.`;
    }
  }

  async function iniciarConversion() {
    if (!filePath) {
      status = "Por favor, selecciona primero un archivo .hbr2";
      return;
    }
    
    status = "Iniciando proceso en el backend...";
    
    try {
      // LLAMADA REAL A RUST:
      // Le pasamos el parámetro "nombreArchivo" tal cual lo declaramos en Rust
      const respuestaRust = await invoke<string>("ejecutar_script", { 
        nombreArchivo: filePath 
      });
      
      status = respuestaRust; // Mostramos lo que nos devuelva Rust
    } catch (error) {
      status = `Error en el backend: ${error}`;
    }
  }
</script>

<main style="padding: 40px; font-family: sans-serif; text-align: center; color: white; background-color: #0f172a; min-height: 100vh;">
  <h2 style="color: #22c55e; font-size: 32px;">HaxRec Converter</h2>
  <p style="color: #94a3b8;">Transformá tus partidas a 60 FPS sin lag</p>
  
  <div style="margin: 30px 0;">
    <input type="file" accept=".hbr2" bind:this={fileInput} on:change={manejarSeleccion} style="display: none;" />
    
    <button on:click={() => fileInput.click()} style="padding: 10px 20px; background-color: #334155; color: white; border: 1px solid #475569; border-radius: 6px; cursor: pointer;">
      {filePath ? "Cambiar Archivo .hbr2" : "Seleccionar Archivo .hbr2"}
    </button>
    <p style="font-size: 13px; color: #64748b; margin-top: 10px;">{filePath || "Ningún archivo seleccionado"}</p>
  </div>

  <button on:click={iniciarConversion} style="padding: 15px 30px; font-size: 18px; background-color: #22c55e; color: white; border: none; border-radius: 8px; font-weight: bold; cursor: pointer; box-shadow: 0 4px 14px rgba(34, 197, 94, 0.4);">
    CONVERTIR HBR2 A MP4
  </button>

  <div style="margin-top: 40px; padding: 15px; background-color: #1e293b; border-radius: 6px; display: inline-block;">
    <span style="color: #94a3b8;">Estado:</span> <span style="color: #fbbf24; font-weight: bold;">{status}</span>
  </div>
</main>