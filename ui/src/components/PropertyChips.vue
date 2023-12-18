<template>
  <template
    v-for="({ name, value }, i) in convertRustObjectToPropertyArray(props.data)"
    :key="i"
  >
    <q-chip dense color="info"> {{ name }}: {{ value }} </q-chip>
  </template>
</template>

<script setup lang="ts">
import { convertRustObjectToPropertyArray } from 'src/services/PropertyHelper';

export interface PropertyChipsProps {
  data?: object;
}
const props = withDefaults(defineProps<PropertyChipsProps>(), {
  data: undefined,
});
</script>

<style scoped lang="scss"></style>
