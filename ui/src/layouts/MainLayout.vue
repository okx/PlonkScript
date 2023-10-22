<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <!-- <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        /> -->

        <q-toolbar-title> Plonk Script </q-toolbar-title>

        <q-btn
          stretch
          :flat="$route.name != 'editor'"
          label="Editor"
          :to="{ name: 'editor' }"
          class="menu-button"
          :class="{ active: $route.name === 'editor' }"
        />
        <!-- <q-separator dark vertical inset /> -->
        <q-btn
          stretch
          :flat="$route.name != 'analyzer'"
          label="Analyzer"
          :to="{ name: 'analyzer' }"
          class="menu-button"
          :class="{ active: $route.name === 'analyzer' }"
        />

        <!-- <div>Demo Version</div> -->
      </q-toolbar>
    </q-header>

    <!-- <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          Essential Links
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer> -->

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
//
</script>

<style lang="scss" scoped>
.menu-button.q-btn {
  background-color: $primary;

  &.active {
    background-color: darken($primary, 10);
  }
}
</style>
