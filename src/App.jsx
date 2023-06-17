import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import ReactPlayer from 'react-player';

function App() {
	const [myStream, setMyStream] = useState(null);

	const startRec = async () => {
		const stream = await navigator.mediaDevices.getUserMedia({
			audio: true,
		});
		setMyStream(stream);
	};

	const stopRec = () => {
		if (myStream) {
			for (let track of myStream.getTracks()) {
				track.stop();
			}
			setMyStream(null);
		}
	};

  useEffect(() => {
    console.log(myStream)
  }, [myStream])

	return (
		<div className="container">
			<h1>Welcome to laugh detector</h1>
			<button type="submit" onClick={startRec}>
				Start recording
			</button>
			{myStream && (
				<>
					<button type="submit" onClick={stopRec}>
						Stop recording
					</button>
					<h1>My Stream</h1>

					<ReactPlayer
						controls={true}
						height="300px"
						width="500px"
						url={myStream}
					/>
				</>
			)}
		</div>
	);
}

export default App;
