import React from 'react';
import '../../Styles/DesignPreview/DesignPreview.css';

function DesignPreview({ generatedImage }) {
    return (
        <div className="design-preview">
            <h2>Vista Previa del Diseño</h2>
            {generatedImage ? (
                <img
                    src={generatedImage}
                    alt="Diseño Generado"
                    className="preview-image"
                />
            ) : (
                <p>Aquí se mostrará la imagen generada por el backend.</p>
            )}
        </div>
    );
}

export default DesignPreview;