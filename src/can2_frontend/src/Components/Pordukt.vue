<template>
    <div class="produkt">
            <div class="holder-produkt-im">
                <!-- <img src="grafika/marsshall-douszne-bezprzewodowe-1.jpg" alt="" class="produkt-img"> -->
            </div>
            <h2 class="produkt-title">MARSHALL Motif II </h2>
            <p class="produkt-def">Lorem ipsum dolor sit amet consectetur adipisicing elit. Corporis quo unde alias beatae accusantium. Maxime neque, assumenda provident quo ipsam possimus libero in, dolore soluta quia quis expedita doloribus omnis!</p>
            <div class="holder-price">
                <span class="produkt-price"> 400 zł</span>
            </div>
            <button class="produkt-addcart">
                <span class="addcart-text">Dodaj do koszyka</span>
                <!-- <i class="fa-solid fa-cart-plus"></i> -->
            </button>
            <!-- <i class="fa-regular fa-circle-xmark close-button"></i> -->
        </div>
</template>

<script>

import { can2_backend } from 'declarations/can2_backend/index';

export default {
    data() {
        return {
            produkcts: [],
            editText: ""
        }
    }
    methods: {
        async getProducts(){
            let tile = await can2_backend.
        }
    },
}

</script>