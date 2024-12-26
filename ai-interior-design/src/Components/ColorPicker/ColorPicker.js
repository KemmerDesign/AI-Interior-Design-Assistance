import React, { useState } from 'react';
import { ChromePicker } from 'react-color';
import '../../Styles/ColorPicker/ColorPicker.css';

function ColorPicker({ onColorSelect }) {
    const [selectedColor, setSelectedColor] = useState('#FFFFFF'); // Color blanco por defecto
    const [showPicker, setShowPicker] = useState(false);

    const handleChange = (color) => {
        setSelectedColor(color.hex);
        onColorSelect(color.hex);
    };

    const handleClick = () => {
        setShowPicker(!showPicker);
    };

    const handleClose = () => {
        setShowPicker(false);
    };

    return (
        <div className="color-picker">
            <h2>Elige un Color para las Paredes</h2>
            <button className='picker-button' onClick={handleClick}>
                {showPicker ? 'Cerrar Selector' : 'Abrir Selector'}
            </button>

            {showPicker && (
                <div className="picker-popover">
                    <div className="picker-cover" onClick={handleClose} />
                    <ChromePicker color={selectedColor} onChange={handleChange} />
                </div>
            )}

            <div
                className="color-preview"
                style={{ backgroundColor: selectedColor }}
            ></div>
        </div>
    );
}

export default ColorPicker;