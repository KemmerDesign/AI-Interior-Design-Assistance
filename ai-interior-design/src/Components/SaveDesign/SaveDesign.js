import React from 'react';
import { FaDownload } from 'react-icons/fa';
import '../../Styles/SaveDesign/SaveDesign.css';

function SaveDesign({ generatedImage }) {
    const handleSave = () => {
        if (generatedImage) {
            // Crear un enlace invisible
            const link = document.createElement('a');
            link.href = generatedImage;
            link.download = 'diseño-interior.jpg'; // Nombre del archivo a descargar
            link.style.display = 'none';
            document.body.appendChild(link);

            // Simular clic en el enlace para iniciar la descarga
            link.click();

            // Eliminar el enlace del DOM
            document.body.removeChild(link);
        }
    };

    return (
        <div className="save-design">
            <button onClick={handleSave} disabled={!generatedImage}>
                <FaDownload className="download-icon" />
                Guardar Diseño
            </button>
        </div>
    );
}

export default SaveDesign;