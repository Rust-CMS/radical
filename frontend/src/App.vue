<template>
  <div id="app">
    <div id="nav">
      <router-link to="/">Home</router-link>
    </div>
    <router-view/>
  </div>
</template>

<style>
@font-face {
  font-family: "montserrat";
  font-style: normal;
  font-weight: 200;
  src: url("./fonts/Montserrat-ExtraLight.ttf");
}
@font-face {
  font-family: "montserrat";
  font-style: normal;
  font-weight: 400;
  src: url("./fonts/Montserrat-Medium.ttf");
}
@font-face {
  font-family: "montserrat";
  font-style: normal;
  font-weight: 600;
  src: url("./fonts/Montserrat-SemiBold.ttf");
}
</style>
