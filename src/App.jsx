import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import ReactPlayer from 'react-player';
import { AudioRecorder } from 'react-audio-voice-recorder';

function App() {
	const [conf, setConf] = useState(null);
	const addAudioElement = (blob) => {
		const url = URL.createObjectURL(blob);
		const audio = document.createElement("audio");
		audio.src = url;
		audio.controls = true;
		document.body.appendChild(audio);
		console.log(url)
		console.log(typeof url)
		invoke("record", {blobUrl: url}).then(resp => {
			setConf(resp)
		})
	};

	return (
		<div className="container">
			<h1>Welcome to laugh detector</h1>
			<AudioRecorder
				onRecordingComplete={addAudioElement}
				audioTrackConstraints={{
					noiseSuppression: true,
					echoCancellation: true,
				}}
				downloadOnSavePress={true}
				downloadFileExtension="mp3"
			/>
			<p>{conf}</p>
		</div>
	);
}

export default App;
