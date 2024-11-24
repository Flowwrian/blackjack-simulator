// eslint-disable-next-line vue/multi-word-component-names
<!-- eslint-disable vue/block-lang -->
<script>


export default {
    data() {
        return {
            balance: 10000,
            dealer: {},
            player: {
                "stats": {}
            },
            stagedBets: 0,
            placedBets: 0,
            cards_remaining: 0,
            game_status: "",
            game_just_finished: false,
            end_response: {}
        }
    },
    watch: {
        game_status(new_status) {
            if (new_status == "PlayerWon" || new_status == "DealerWon" || new_status == "Draw") {
                this.game_just_finished = true;
                fetch("http://127.0.0.1:8000/end", {
                    method: "POST",
                    body: JSON.stringify({ action: new_status })
                }).then((response) => {
                    response.json().then((content) => {
                        this.end_response = content;
                        this.cards_remaining = content.cards_remaining;
                    }).catch((err) => console.log(err));
                }).catch((err) => console.log(err));
            }
        }
    },
    methods: {
        playerHasCards() {
            const res = this.player.hand && this.player.hand.length > 0;
            return res;
        },
        calculateHandValue(hand) {
            if (hand == undefined || hand.length == 0) {
                return null;
            } else {
                let hand_value = 0;
                // add all non-ace cards together
                for (const card of hand) {
                    if (card.value != "Ace") {
                        hand_value += card.numeric_value;
                    }
                }

                // add all aces
                for (const card of hand) {
                    if (card.value == "Ace") {
                        if (hand_value + 11 <= 21) {
                            hand_value += 11;
                        } else {
                            hand_value += 1;
                        }
                    }
                }
                return hand_value;
            }
        },
        playAction(action) {
            if (this.game_status == "Ongoing") {
                fetch("http://127.0.0.1:8000/action", {
                    method: "POST",
                    body: JSON.stringify({ action: action })
                }).then((response) => {
                    response.json().then((content) => {
                        this.dealer = content.dealer;
                        this.player = content.player;
                        this.placedBets = content.bets;
                        this.cards_remaining = content.cards_remaining;
                        this.game_status = content.game_status;
                    }).catch((err) => console.log(err));
                }).catch((err) => console.log(err));
            }
        },
        resetGame() {
            // write results
            this.placedBets = 0; // since the game is over, this value is 0 again
            this.game_status = this.end_response.game_status;
            this.player = this.end_response.player;
            this.dealer = this.end_response.dealer;

            this.game_just_finished = false;
        },
        initalFetch() {
            fetch("http://127.0.0.1:8000/init", {
                method: "GET",
            }).then((response) => {
                response.json().then((content) => {
                    this.dealer = content.dealer;
                    this.player = content.player;
                    this.placedBets = content.bets;
                    this.cards_remaining = content.cards_remaining;
                    this.game_status = content.game_status;
                }).catch((err) => console.log(err));
            }).catch((err) => {
                console.log(err);
            })
        },
        simulateDealer() {
            fetch("http://127.0.0.1:8000/simulateDealer", {
                method: "GET",
            }).then((response) => {
                response.json().then((content) => {
                    this.dealer = content.dealer;
                    this.player = content.player;
                    this.placedBets = content.bets;
                    this.cards_remaining = content.cards_remaining;
                    this.game_status = content.game_status;
                }).catch((err) => console.log(err));
            }).catch((err) => {
                console.log(err);
            })
        },
        placeBet() {
            // betting should only be possible at the start of the game
            if (this.game_status != "Initalized" || this.stagedBets == 0) {
                return;
            }

            fetch("http://127.0.0.1:8000/startGame", {
                method: "POST",
                body: JSON.stringify({ amount: this.stagedBets })
            }).then((response) => {
                response.json().then((content) => {
                    this.dealer = content.dealer;
                    this.player = content.player;
                    this.placedBets = content.bets;
                    this.cards_remaining = content.cards_remaining;
                    this.game_status = content.game_status;
                }).catch((err) => console.log(err));
            }).catch((err) => console.log(err));
        },
        printCard(cardObject) {
            let color;
            let value;
            switch (cardObject.color) {
                case "Diamonds":
                    color = "♦";
                    break;
                case "Hearts":
                    color = "♥";
                    break;
                case "Clubs":
                    color = "♣";
                    break;
                case "Spades":
                    color = "♠";
                    break;
            }

            switch (cardObject.value) {
                case "Two":
                    value = "2";
                    break;
                case "Three":
                    value = "3";
                    break;
                case "Four":
                    value = "4";
                    break;
                case "Five":
                    value = "5";
                    break;
                case "Six":
                    value = "6";
                    break;
                case "Seven":
                    value = "7";
                    break;
                case "Eight":
                    value = "8";
                    break;
                case "Nine":
                    value = "9";
                    break;
                case "Ten":
                    value = "10";
                    break;
                case "Jack":
                    value = "J";
                    break;
                case "Queen":
                    value = "Q";
                    break;
                case "King":
                    value = "K";
                    break;
                case "Ace":
                    value = "A";
                    break;
            }
            return `${value}${color}`
        },
        confirmBet(amount) {
            if (this.game_status == "Initalized") {
                this.stagedBets = amount;
            }
        }
    },

    mounted() {
        this.initalFetch()
    }
}
</script>

<template>
    <div class="Board">
        <!-- Betting panel -->
        <div class="SidePanel">
            <div class="UserButton" @click="confirmBet(10)">10</div>
            <div class="UserButton" @click="confirmBet(50)">50</div>
            <div class="UserButton" @click="confirmBet(100)">100</div>
            <div class="UserButton" @click="confirmBet(1000)">1000</div>
            <input type="number" v-model="this.stagedBets" min="1" max="1000000" placeholder="Enter value"
                :disabled="game_status != 'Initalized'">
            <div class="UserButton" id="PlaceBet" @click="placeBet">Place bet</div>
        </div>
        <!-- Board -->
        <div class="BoardContent">
            <div class="BalanceContainer">
                <span>Balance:</span>
                <div class="BalanceValue">{{ player.balance }}</div>
                <span>Bet:</span>
                <div class="BalanceValue">{{ placedBets }}</div>
            </div>
            <h1>Dealers Hand</h1>
            <div class="GameBar">
                <div class="HandRow">
                    <template v-if="this.dealer.hand && this.dealer.hand.length > 0">
                        <span v-for="card in dealer.hand" :key="card">{{ `${printCard(card)} ` }}</span>
                    </template> <template v-else>
                        No cards
                    </template>
                </div>
                <div class="HandValue">
                    <span v-if="calculateHandValue(dealer.hand) != null">{{ calculateHandValue(dealer.hand) }}</span>
                </div>
            </div>
            <h1>Your Hand</h1>
            <div class="GameBar">
                <div class="HandRow">
                    <template v-if="this.player.hand && this.player.hand.length > 0">
                        <span v-for="card in player.hand" :key="card">{{ `${printCard(card)} ` }}</span>
                    </template> <template v-else>
                        No cards
                    </template>
                </div>
                <div class="HandValue">
                    <span v-if="calculateHandValue(player.hand) != null">{{ calculateHandValue(player.hand) }}</span>
                </div>
            </div>
            <!-- Buttons for user actions -->
            <br>
            <div class="UserButtonContainer">
                <div class="UserButton" @click="playAction('Hit')">Hit</div>
                <div class="UserButton" @click="playAction('Stand')">Stand</div>
                <div class="UserButton" @click="playAction('Double')">Double</div>
                <div class="UserButton" @click="simulateDealer" v-if="game_status == 'PlayerFinished'">Dealers Turn
                </div>
                <dir class="UserButton" @click="resetGame" v-if="game_just_finished">Start new game</dir>
            </div>
        </div>
        <!-- Statistics side panel -->
        <div class="SidePanel" id="StatisticsPanel">
            <h1>Statistics</h1>
            <div class="InfoPanelContent">
                <div>{{ `Average Bet: ${player.stats.average_bet}` }}</div>
                <div>{{ `Average Payout: ${player.stats.average_win}` }}</div>
                <div>{{ `Matches Played: ${player.stats.matches_played}` }}</div>
                <div>{{ `Times Doubled: ${player.stats.times_doubled}` }}</div>
                <div>{{ `Card Count: ${player.stats.card_count}` }}</div>
            </div>
        </div>
    </div>


</template>

<style>
.GameBar {
    display: flex;
    flex-wrap: nowrap;
    justify-content: start;
    align-items: center;
    gap: 3vh;
}

.HandValue {
    font-size: 150%;
}

.SidePanel {
    height: 100%;
    border: 2px solid black;
    border-radius: 10px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: start;
    align-content: space-evenly;
    gap: 5px;
    width: 1000px;

    #PlaceBet {
        margin-top: 1.5vh;
        padding: 0;
        text-align: center;
    }

}

.BalanceContainer {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    align-items: baseline;

    span {
        position: relative;
        font-size: 250%;
    }

    margin-bottom: 8%;

    .BalanceValue {
        margin-left: 1vh;
        margin-right: 1vh;
        font-size: 180%;
    }
}

.Board {
    position: fixed;
    left: 28%;
    top: 20%;
    height: 25vh;
    width: 80vh;
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: space-around;
    align-items: center;
    gap: 5vh;

    h1 {
        margin-bottom: 1px;
    }

    span {
        word-spacing: 10px;
    }
}

.UserButtonContainer {
    margin-top: 1vh;
    width: 30vh;
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: space-between;
}

.UserButton {
    border: 1px solid black;
    border-radius: 10px;
    padding: 5px 10px 5px 10px;
    user-select: none;
}

#StatisticsPanel {
    h1 {
        text-align: center;
    }
}
</style>