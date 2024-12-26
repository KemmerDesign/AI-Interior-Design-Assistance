import React, { useState } from "react";
import { FaUpload} from "react-icons/fa"; //Este es un icono de React
import '../../Styles/RoomImageUpload/RoomImageUpload.css';

function RoomImageUpload({onImageUpload}) {
  const [selectedFile, setSelectedFile] = useState(null);
  const [previewUrl, setPreviewUrl] = useState(null);

  const handleFileChange = (event) => {
    const file = event.target.files[0];
    if (file){
        setSelectedFile(file);
        setPreviewUrl(URL.createObjectURL(file));
        onImageUpload(file);
    }
  };

  return (
    <div className="upload-container">
        <label htmlFor="file-upload" className="upload-label">
            <FaUpload className="upload-icon"/>
            <span>Subir imagen de la habitación</span>
        </label>
        <input type="file" id="file-upload" onChange={handleFileChange}/>
        {previewUrl && (
            <div className="preview-container">
                <img src={previewUrl} alt="Vista previa" className="preview-image"/>
            </div>
        )}
    </div>
  );
}

export default RoomImageUpload;