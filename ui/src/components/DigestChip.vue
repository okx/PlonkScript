<template>
  <q-chip dense color="info">
    {{ name }}:
    {{ hashoutToHex(props.data) }}
  </q-chip>
</template>

<script setup lang="ts">
import { HashOut } from 'src/services/plonky2/DefaultModels';
import { hashoutToHex } from 'src/services/PropertyHelper';

export interface ChipProps {
  name?: string;
  data?: HashOut;
}
const props = withDefaults(defineProps<ChipProps>(), {
  name: '',
  data: undefined,
});
</script>

<style scoped lang="scss"></style>
