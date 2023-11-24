<script setup>
const props = defineProps({
  selected: {default: () => (0) }
})

const options = [
  { name: 'select text', id: 0 },
  { name: 'book1', id: 1 },
  { name: 'book2', id: 2 },
  { name: 'book3', id: 3 }
];
const emit = defineEmits(['update:selected'])

function inputSignal(e) {
  emit('update:selected',e.target.value)
}
</script>

<template>
  <div class="container">
    <select id="select" @change="inputSignal" :value="selected">
        <option v-for = "option in options" :value="option.id">{{ option.name }}</option>
    </select>
    <input id="input" type="text" :value="selected">
    <button id="button">Save</button>
  </div>
</template>

