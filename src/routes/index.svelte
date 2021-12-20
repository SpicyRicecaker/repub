<script lang="ts">
	import { onMount } from 'svelte';

	const tauri = (window as any).__TAURI__ as typeof import('@tauri-apps/api/tauri');

	let html = 'test';

	onMount(async () => {
		// load config file, pref serde
		console.log('debug');
		window.addEventListener('keydown', handleKeyDown);
		await tauri.invoke('init_book');
		html = await tauri.invoke('load_chapter');
	});

	const handleKeyDown = async (e: KeyboardEvent) => {
		console.log('keypressed');
		switch (e.key) {
			case 'ArrowLeft': {
				await tauri.invoke('change_chapter', { change: -1 });
				html = await tauri.invoke('load_chapter');
				break;
			}
			case 'ArrowRight': {
				await tauri.invoke('change_chapter', { change: 1 });
				html = await tauri.invoke('load_chapter');
				break;
			}
			default: {
				console.log(e.key);
				break;
			}
		}
	};

	// let edit_mode: boolean = false;

	// const save = async () => {
	// 	// update
	// 	if (edit_mode) {
	// 		actions = actions;
	// 		// export to rust
	// 		let res = await tauri.invoke('save', { actions: JSON.stringify(actions) });
	// 	}
	// 	edit_mode = !edit_mode;
	// };

	// const launch = async () => {
	// 	// save all changes before
	// 	actions = actions;
	// 	let _ = await tauri.invoke('save', { actions: JSON.stringify(actions) });

	// 	let res = await tauri.invoke('run');
	// 	console.log(res);
	// };
</script>

<main>
	{@html html}
</main>

<style lang="scss">
	:global(#svelte, html, body) {
		width: 100%;
		height: 100%;
	}
	:global(body) {
		margin: 0;
	}

	main {
		width: 100%;
		height: 100%;
	}
</style>
