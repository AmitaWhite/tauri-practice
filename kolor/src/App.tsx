import { invoke } from "@tauri-apps/api/core";
import { SketchPicker } from "react-color";
import "./App.css";
import { useState } from "react";

function App() {
	const [color, setColor] = useState("#1fa9f4");
	const [gradient, setGradient] = useState<number[][]>([]);

	return (
		<div className="container">
			<SketchPicker
				color={color}
				onChange={(pickerColor) => {
					setColor(pickerColor.hex);
					invoke<number[][]>("generate_gradient", {
						r: pickerColor.rgb.r,
						g: pickerColor.rgb.g,
						b: pickerColor.rgb.b,
					}).then((grad) => {
						setGradient(grad);
					});
				}}
			/>
			{gradient.map((rgbArray) => {
				const colorId = rgbArray.join("-");
				return (
					<div
						key={colorId}
						style={{
							padding: "2rem",
							background: `rgb(${rgbArray[0]} , ${rgbArray[1]},${rgbArray[2]})`,
						}}
					>
						rgb({rgbArray[0]},{rgbArray[1]},{rgbArray[2]})
					</div>
				);
			})}
		</div>
	);
}

export default App;
