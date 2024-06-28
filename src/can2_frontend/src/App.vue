<script setup>
import { ref } from 'vue';
import { can2_backend } from 'declarations/can2_backend/index';
let greeting = ref('');

import Produkt from "./Components/Pordukt.vue";


async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await can2_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
    <header id="header">
        <h1 id="title">
            <i class="fa-solid fa-headphones"></i>
            <span>Maloman.com</span>
        </h1>
        <!-- <div id=""> -->
            <label id="search-holder">
                <i class="fa-solid fa-magnifying-glass" id="glass"></i>
                <input type="search" name="search" id="search" placeholder="Wyszukaj w naszym sklepie...">
            </label>
        <!-- </div> -->
        <div id="icons">
            <div id="cart-holder" class="icon">
                <i class="fa-solid fa-cart-shopping icon"></i>
                <div id="cart-counter"></div>
                <div id="cart-list" class="list">
                    <div id="clear-cart"><span>Wyczyść koszyk</span></div>
                </div>
            </div>
                        
            <i id="favorite" class="fa-regular fa-heart icon"></i>
            
            <i id="accout" class="fa-regular fa-user icon"></i>
        </div>
        <div id="icon-small">
            <i class="fa-solid fa-bars"></i>
        </div>
    </header>
    <nav id="categories">
        <!-- tu kategoria np. marki albo typy, pomysł porzucony, może jak będzie czas-->
    </nav>
    <div id="holder-products">
      <Produkt />
    </div>
</template>
