<script lang="ts">
    export let prefix: string;
    import { open } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';
    async function select(){
        // Open a selection dialog for image files
        const selected = await open({
            multiple: false,
            filters: [{
                name: 'Executable',
                extensions: ['exe', 'msi']
            }]
        });
        if (selected === null) {
        // user cancelled the selection
        } else {
            invoke("open_app_in_prefix", {path: selected, prefix: prefix})
        }
    }
</script>
<p>Run application in Wineprefix</p>
<div class="d-flex justify-content-center">
    <button class="btn btn-primary" on:click={select}>Browse file</button>
</div>
<br/>
<div class="d-flex justify-content-center">
    <button class="btn btn-primary" on:click={() => invoke('cfg_prefix',{prefix: prefix})}>Open WineCFG</button>
</div>