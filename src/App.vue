<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  data() {
    return {
      selected: [],
    }
  },
  methods: {
    async save() {
      await invoke('save');
    }
  },
  mounted() {
    this.save();
  }
}

</script>

<template>
  <div class="container">
    <select id="select" :on-change="selected">
      <option hidden>Select item</option>
      <option value="1">One</option>
      <option value="2">Two</option>
    </select>

    <input id="input" type="text" :value="selected" placeholder="Select item">

    <button id="button" @click="save">Save</button>
  </div>
</template>

