<script setup>

import { save } from '@tauri-apps/api/dialog';
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const selected = ref('enter');
const options = [
  { name: 'select text', id: 'enter' },
  { name: 'book1', id: 'a'} ,
  { name: 'book2', id: 'b'},
  { name: 'book3', id: 'c'}
];

const message = ref(selected);

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
        <option v-for = "option in options" :value="option.id">{{ option.name }}</option>
    </select>

    <input id="input" type="text" v-model="selected">

    <button id="button" @click="saveFileContents">Save file</button>

  </div>
</template>

