import React, { useState } from 'react';
import '../../Styles/FurnitureSelector/FurnitureSelector.css';

const furnitureOptions = [
    {
        type: 'Sofá',
        items: [
            { name: 'Sofá Chesterfield', image: '/images/sofa_chesterfield.jpg' },
            { name: 'Sofá Seccional', image: '/images/sofa_seccional.jpg' },
            { name: 'Sofá Cama', image: '/images/sofa_cama.jpg' },
        ],
    },
    {
        type: 'Mesa',
        items: [
            { name: 'Mesa de Centro', image: '/images/mesa_centro.jpg' },
            { name: 'Mesa de Comedor', image: '/images/mesa_comedor.jpg' },
            { name: 'Escritorio', image: '/images/escritorio.jpg' },
        ],
    },
    {
        type: 'Silla',
        items: [
            { name: 'Silla Eames', image: '/images/silla_eames.jpg' },
            { name: 'Silla de Comedor', image: '/images/silla_comedor.jpg' },
            { name: 'Sillón', image: '/images/sillon.jpg' },
        ],
    },
];

function FurnitureSelector({ onFurnitureSelect }) {
    const [selectedFurnitureType, setSelectedFurnitureType] = useState(null);
    const [selectedFurniture, setSelectedFurniture] = useState(null);

    const handleFurnitureTypeClick = (furnitureType) => {
        setSelectedFurnitureType(furnitureType);
        setSelectedFurniture(null); // Reiniciar la selección de mueble al cambiar de tipo
    };

    const handleFurnitureClick = (furniture) => {
        setSelectedFurniture(furniture);
        onFurnitureSelect(furniture);
    };

    return (
        <div className="furniture-selector">
            <h2>Elige Muebles</h2>
            <div className="furniture-types">
                {furnitureOptions.map((type) => (
                    <button
                        key={type.type}
                        className={`furniture-type ${selectedFurnitureType === type.type ? 'selected' : ''}`}
                        onClick={() => handleFurnitureTypeClick(type.type)}
                    >
                        {type.type}
                    </button>
                ))}
            </div>

            {selectedFurnitureType && (
                <div className="furniture-items">
                    {furnitureOptions
                        .find((type) => type.type === selectedFurnitureType)
                        .items.map((item) => (
                            <div
                                key={item.name}
                                className={`furniture-item ${selectedFurniture === item ? 'selected' : ''}`}
                                onClick={() => handleFurnitureClick(item)}
                            >
                                <img src={item.image} alt={item.name} />
                                <p>{item.name}</p>
                            </div>
                        ))}
                </div>
            )}
        </div>
    );
}

export default FurnitureSelector;