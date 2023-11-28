<script setup>

import { save } from '@tauri-apps/api/dialog';
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const selected = ref('enter');
const options = ref([]);

const message = ref(selected);

onMounted(()=>{
  invoke('menu_open').then((obj)=>{
    
    const menu=JSON.parse(obj);
    options.value = menu.menu;
    console.log(options.value);
  })
})


const saveFileContents = async () => {
  try {
    const savePath = await save();
    if (!savePath) return;
    await invoke("save_file", { path: savePath, contents: message.value });
  } catch(err) {
    console.error(err);
  }
}

</script>

<template>
  <div class="container">

    <select id="select" v-model="selected">
        <option v-for = "option in options" :value="option.name">{{ option.name }}</option>
    </select>

    <input id="input" type="text" v-model="selected">

    <button id="button" @click="saveFileContents">Save file</button>

  </div>
</template>

