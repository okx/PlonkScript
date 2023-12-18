<template>
  <template
    v-for="(p, i) in filterRustObjectWithArrayProperties(props.data)"
    :key="i"
  >
    {{ p.name }}: [{{ p.value.length }}]
    <array-chips :data="p.value" />
  </template>
</template>

<script setup lang="ts">
import { filterRustObjectWithArrayProperties } from 'src/services/PropertyHelper';
import ArrayChips from './ArrayChips.vue';

export interface PropertyChipsProps {
  data?: object;
}
const props = withDefaults(defineProps<PropertyChipsProps>(), {
  data: undefined,
});
</script>

<style scoped lang="scss"></style>
