import React, {useState} from "react";
import './App.css';
import RoomImageUpload from './Components/RoomImageUpload/RoomImageUpload';
import StyleSelector from "./Components/Style selector/StyleSelector";
import FurnitureSelector from "./Components/FurnitureSelector/FurnitureSelector";
import ColorPicker from "./Components/ColorPicker/ColorPicker";
import DesignPreview from "./Components/DesignPreview/DesignPreview";
import SaveDesign from './Components/SaveDesign/SaveDesign'; // Importamos SaveDesign


function App() {
  const [imageData, setImageData] = useState(null);
  const [selectedStyle, setSelectedStyle] = useState(null);
  const [selectedFurniture, setSelectedFurniture] = useState(null);
  const [selectedColor, setSelectedColor] = useState('#FFFFFF');
  const [generatedImage, setGeneratedImage] = useState(null); // Nuevo estado para la imagen generada

  const handleImageUpload = (data) => {
    setImageData(data);
    //Punto de envio de la imagen al backen
    console.log('Imagen recibida en App.js',data);
  };
  const handleStyleSelect = (styleName) => {
    setSelectedStyle(styleName);
    //Punto de envio del estilo al backend
    console.log('Estilo seleccionado en App.js',styleName);
  };
  const handleFurnitureSelect = (furniture) => {
    setSelectedFurniture(furniture);
    console.log('Mueble seleccionado en App.js:', furniture);
    // Aquí enviaríamos el mueble seleccionado al backend
  };
  const handleColorSelect = (color) => {
    setSelectedColor(color);
    console.log('Color seleccionado en App.js:', color);
    // Aquí enviaríamos el color seleccionado al backend
  };
  const handleGenerateImage = () => {
    // Aquí iría la llamada al backend para generar la imagen
    // Por ahora, simulamos una respuesta después de 2 segundos
    setTimeout(() => {
      setGeneratedImage('/images/imagen_generada.jpg'); // Reemplaza con una URL de imagen válida
    }, 2000);
  };
  return (
      <div className="App">
        <h1>AI Interior Design</h1>
        <RoomImageUpload onImageUpload={handleImageUpload}/>
        <StyleSelector onStyleSelect={handleStyleSelect}/>
        <FurnitureSelector onFurnitureSelect={handleFurnitureSelect}/>
        <ColorPicker onColorSelect={handleColorSelect}/>
        <button onClick={handleGenerateImage}>Generar Imagen</button>
        {/* Es el trigger para que funcione todo*/}
        <DesignPreview generatedImage={generatedImage}/>
          <SaveDesign generatedImage={generatedImage} />
      </div>
  );
}

export default App;
