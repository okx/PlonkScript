<template>
  <div style="max-width: 800px">
    <q-virtual-scroll
      :items="props.data"
      virtual-scroll-horizontal
      v-slot="{ item, index }"
    >
      <q-badge
        outline
        :color="index % 2 == 0 ? 'primary' : 'secondary'"
        class="q-mx-xs"
      >
        <template v-slot>
          <span class="minor">{{ index }}:</span>
          {{ item }}
        </template>
      </q-badge>
    </q-virtual-scroll>
  </div>
</template>

<script setup lang="ts">
export interface PropertyChipsProps {
  data?: string[];
}
const props = withDefaults(defineProps<PropertyChipsProps>(), {
  data: undefined,
});
</script>

<style scoped lang="scss">
span.minor {
  opacity: 50%;
  font-size: 0.6rem;
}
</style>
