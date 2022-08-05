<script lang="ts">
    import 'bootstrap/dist/css/bootstrap.css';
    import 'bootstrap/dist/js/bootstrap.js';
    import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
    import { Circle2 } from 'svelte-loading-spinners';
       let apps: object = [], loading = true, installing = false;
    let current_app: string, current_description: string, current_author: string, current_icon: string, current_url: string, downloading = false;
    onMount(async () => {
		apps = await invoke('update_repo', {url: "https://raw.githubusercontent.com/Windows-On-Linux/Repo/main/repository.json"});
        loading = false;
    });

    function installapp(name: string, description: string, author: string, icon: string, url: string){
        installing = true;
        current_app = name;
        current_description = description;
        current_author = author;
        current_icon = icon;
        current_url = url;
    }

    async function install(){
        downloading = true;
        let result = await invoke("download_script", {url: current_url, path: "~/wol/Downloads", name: current_app});
        downloading = false;
        if(!result){
            alert("Error during downloading of script");
        }else{
            invoke("execute_script", {name: current_app});
            alert("The installation is started, check the progress in terminal");
            location.href = "/repository";
        }
    }
</script>

{#if installing == false}
{#if loading == true}
<div class="d-flex justify-content-center mt-5">
    <Circle2 size="256" unit="px"></Circle2>
</div>
{:else if loading == false}
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
    {#each apps as app}
        <div class="list-group m-4">
            <a href="#0" class="list-group-item " aria-current="true">
            <div class="d-flex w-100 justify-content-between">
                <a href={app.Repository} target="_blank">
                    <h5 class="mb-1">{app.App}</h5>
                </a>
                <small>{app.Version}</small>
                <button class="btn btn-primary" on:click={() => installapp(app.App, app.Description,app.Author, app.Icon, app.Repository)}>Install</button>
            </div>
            <p class="mb-1">{app.Description}</p>
            <small>{app.Author}</small>
            </a>
        </div>
    {/each}
{/if}
{:else}
<div class="m-5">
    <div class="d-flex justify-content-between">
    <div>
        <h1>{current_app}</h1>
        <small>{current_description}</small>
        <br/>
        <small>{current_author}</small>
    </div>
    <img src={current_icon} alt="Icon" class="icon" width="128px"/>
</div>
<br/>
<div class="d-flex justify-content-center mt-2">
    <p>Are you sure to install {current_app}?</p>
</div>
<div class="d-flex justify-content-center">
    <button class="btn btn-primary m-3" on:click={install}>Yes</button>
    <br/>
    <br/>
    <button class="btn btn-danger m-3" on:click={() => installing = false}>No</button>

</div>
{#if downloading == true}
<br/>
<br/>
<div class="d-flex justify-content-center">
    <Circle2 size="256" unit="px"></Circle2>
</div>
{/if}
</div>

{/if}

<style>
    a {
        color: black;
        text-decoration: none;
    }
    .icon{
        margin-top: -40px;
        float: right;
    }
</style>