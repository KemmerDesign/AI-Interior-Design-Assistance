import React, {useState} from "react";
import '../../Styles/Style selector/StyleSelector.css';

const styleOptions = [
    {name: 'Moderno', image: '/images/moderno.jpg'},
    { name: 'Rústico', image: '/images/rustico.jpg' },
    { name: 'Minimalista', image: '/images/minimalista.jpg' },
    { name: 'Industrial', image: '/images/industrial.jpg' },
    { name: 'Bohemio', image: '/images/bohemio.jpg' },
];

function StyleSelector({ onStyleSelect }){
    const [selectedStyle, setSelectedStyle] = useState(null);
    const handleStyleClick = (styleName) => {
        setSelectedStyle(styleName);
        onStyleSelect(styleName);
    };
    //TODO estudiar mejor esta parte del codigo
    return(
        <div className="style-selector">
            <h2>Elige un Estilo</h2>
            <div className="style-options">

                {styleOptions.map((style)=>(
                    <div key={style.name}
                    className={`style-option ${selectedStyle === style.name ? 'selected' : ''}`}
                    onClick={() => handleStyleClick(style.name)}>
                        <img src={style.image} alt = {style.name} />
                        <p>{style.name}</p>
                    </div>
                ))}
            </div>
            </div>
    );
}

export default StyleSelector;