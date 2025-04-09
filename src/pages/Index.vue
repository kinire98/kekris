<template>
  <div @click="">
	  <img src="/logo-drop-shadow.png" alt="">
	  <h2>Click anywhere</h2>
  </div>
</template>
<style scoped>
div {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 85vh;
	width: 100vw;
	flex-direction: column;
	* {
		margin-block: 5rem;
	}
}	
h2 {
	animation: opa 2s ease-in infinite;
}
@keyframes opa {
	0% {
		opacity: 1;
	}	
	50% {
		opacity: 0;
	}
	100% {
		opacity: 1;
	}
}

</style>

<script lang="ts">
	export default {
		methods: {
			changeRoute() {
    			this.$router.push("/main"); 
			}
		},
		mounted() {
			document.body.addEventListener("click", this.changeRoute)
		},
		beforeDestroy() {
			document.body.removeEventListener("click", this.changeRoute);
		}
	}
</script>