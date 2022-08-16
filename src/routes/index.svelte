<script lang="ts">
    import { dialogs } from 'svelte-dialogs';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import 'bootstrap/dist/css/bootstrap.css';
    import 'bootstrap/dist/js/bootstrap.js';
    import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
    import Fa from 'svelte-fa/src/fa.svelte'
    import { faPlay, faGear } from '@fortawesome/free-solid-svg-icons/index.es';
    import Setting from './lib/setting.svelte';
    let dir: Array<object> = [];
    let dir2: Array<object> = [];
    onMount(async () => {
      await invoke('initapp');
      dir = await invoke('read_dir');
      for(let i = 0; i < dir.length; i++) {
        dir2.push({
          name: await readTextFile(dir[i].path, { dir: BaseDirectory.Home }),
          startup: dir[i].startup,
          prefix: dir[i].path.replace("name.txt", "")
        })
      }
      dir2 = dir2;
      console.log('Init application completed');
    });
</script>
<nav class="navbar navbar-expand-lg bg-light">
    <div class="container-fluid">
      <a class="navbar-brand" href="#">Windows on Linux</a>
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <a class="nav-link active" aria-current="page" href="/">Home</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="/repository">Repository</a>
          </li>
      </div>
    </div>
  </nav>

  <ol class="list-group list-group-numbered m-5">
    {#each dir2 as app}
    <li class="list-group-item d-flex justify-content-between align-items-start">
      <div class="ms-2 me-auto">
        <div class="fw-bold">{app.name}</div>
      </div>
      <span class="" on:click={() => invoke('open_app', {path: app.startup})} style="margin-right: 10px;">
        <Fa icon={faPlay} color="green"/>
      </span>
      <span class="" on:click={() => dialogs.modal(Setting, {prefix: app.prefix})}>
        <Fa icon={faGear} color="green"/>
      </span>
    </li>
  {/each}
  </ol>