import { Suspense, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import { AudioRecorder } from 'react-audio-voice-recorder';

function App() {
	const [conf, setConf] = useState(null);
	const [url, setUrl] = useState(null);
	const addAudioElement = (blob) => {
		const url = URL.createObjectURL(blob);
		setUrl(url);
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
			{url ? (
				<>
					<audio src={url} controls={true} />
					<Suspense fallback={<p>Loading...</p>}>
						{conf ? <p>{conf}</p> : null}
					</Suspense>
				</>
			) : null}
		</div>
	);
}

export default App;
