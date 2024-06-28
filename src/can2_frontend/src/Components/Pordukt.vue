<template>
    <div id="products" v-for="product in produkcts" >
        <div class="produkt">
            <div class="holder-produkt-im">
                <img :scr="product.url" alt="" class="produkt-img">
            </div>
            <h2 class="produkt-title">{{ product["title"] }}</h2>
            <p class="produkt-def">{{ product['desc'] }}</p>
            <div class="holder-price">
                <span class="produkt-price">{{ product["price"] }}</span>
            </div>
            <button class="produkt-addcart" @click="getProducts">
                <span class="addcart-text">Dodaj do koszyka</span>
                <!-- <i class="fa-solid fa-cart-plus"></i> -->
            </button>
            <!-- <i class="fa-regular fa-circle-xmark close-button"></i> -->
        </div>
      </div>
        {{ produkcts[0] }}
</template>

<script>

import { can2_backend } from 'declarations/can2_backend/index';

export default {
    data() {
        return {
            produkcts: [],
            editText: ""
        }
    },
    methods: {
        async getProducts(){
            let title = await can2_backend.read_title();
            let price = await can2_backend.read_price();
            let desc = await can2_backend.read_desc();
            let url = await can2_backend.read_url();
            
            for (let index = 0; index < title.length; index++) {
                this.produkcts[index] = {"title": title[index],"desc": desc[index],"price": price[index],"url": url[index]}
            }

        }
    },
    async mounted() {
        this.getProducts()
    },
}

</script>