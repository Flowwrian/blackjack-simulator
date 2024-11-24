<!-- eslint-disable vue/block-lang -->
<script>
import Board from './components/Board.vue';

export default {
  data() {
    return {
      
    }
  },
  methods: {
    
  },

  mounted() {
    console.log(`Component loaded`)
  },
  components: {
    Board: Board
  }
}
</script>

<template>
  <Logo />
  <Board />
</template>