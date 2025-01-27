# Blackjack AI

This a simple AI I have written to play blackjack against the player. I used reinforment learning (RL) as I thought it would be the best approach.

## How does this work?

For those unaware, the way RL works is that I start off with an AI that doesnt even know the rules of blackjack. It will select hit and stay at random.

This seems counter intuivie at first, but if the AI does manage to win, we give a little reward, and if it loses we punish it. Over time the AI will learn what moves it should be doing because of sitations it has been in before and slowly, the AI will get better and better.

## The Twist

As many people are aware, like many casino games, blackjack is a losing game. Thats how it was designed.

This means that under normal circumstances, the AI will never surpass the dealer, however the AI can also learn to count cards--BUT it doenst know how to do that, and will learn over time.

# Q-Learning

People like to pretend that AI is unreasonibly complicated thing, but from my expirence, thats only half true.

## Overview

The overal premis is childishly simplistic--the AI does something good: give it a reward, if does something bad: punish it.

I have ommited the part about counting cards from the overview, but if you are curious, its in the technical section.

### The Process

Essentially it goes like this:
1. The AI plays games.
2. After each game, it updates its knowledge (stored in the Q-table/CSV) based on what happened.
3. If it made a good decision (like hitting and then winning), the Q-value for that action in that state will increase.
4. If it made a bad decision (like hitting and losing), the Q-value for that action in that state will decrease.
5. Over time, the AI learns the best actions to take in various situations (like which cards to hit or stand on) based on past experience.

### How it plays out

**First Game:**
The AI starts with a random Q-table (or zeros).
It tries an action (like hitting with a hand of 14). It doesn't know if that's good or bad.
Let's say it loses the game. It updates the Q-value for that action in that state (e.g., "hit on 14") to reflect the bad outcome (maybe the Q-value goes down).

**Second Game:**
The AI tries again, using the updated Q-table from its previous game.
This time, it might take a different action (like standing on 14) and win. Now, it updates the Q-value for "stand on 14" to reflect the good outcome (the Q-value goes up).

**Third Game:**
As it plays more games, the Q-values become more refined, and it starts preferring the actions that lead to better results, like standing on 20 or hitting on low values when the card count is favorable.

## Technical

I know the overview isn't enough for some people so I have made a more detailed discription of what I have done for those who are interested.

### Setting up the base game

This was the easist part. I just coded a simple blackjack game with rust and forced a randomized bot to play through it a couple (thousand) times just to make sure everything was running smoothly.

### Adding the Q-Learning Model

First I need to define the State Space. A state is a description of the sitation the AI is currently in. I started with a simple state representation; Hand Value (total value of the player’s cards) and Dealer’s Visible Card (the face-up card). This state can be represented as The state can be represented as `(Hand Value, Dealer’s Card)`.

Next I need to define the Action Space. As the name would imply, an Action is what the agent can do. For now, I just need a basic agent with two actions; hit and stand.

Then I made a simple reward system to make the AI improve. The reward is +1 if it wins, -1 if it loses and 0 if its a tie.

Finally I need to implement the Q-Learning algorithim. I initialize the Q-table for each action-state pair `(Hand Value, Dealer’s Card)`, and initialize the Q-values to 0. A Q-table (in most cases) is just a .csv file with a bunch of weights. The AI then uses those weights to see what happened in the passed now is going to use that data to try and predict a better move, thus rewarding it with more reward. A Q-value is a number that represents how good it is to take a certain action in a certain senario. The Q-value is a measure of the expected future reward. AI will learn to update these values based on previous outcomes.

I used epsilon-greedy approach where the agent explores with probability $ϵ$ and exploits with probability $1 − ϵ$. Epsilon-greedy is a policy which that AI uses to stragize. The AI follows—essentially, which action to take in a given state. Initially, the policy is random, but over time it improves as the AI updates the Q-values.

And then I updated the Q-value implementing the following Q-learning update rule:

$$
Q(s_{t}​,a_{t}​) = Q(s_{t}​,a_{t}​)+α(r_{t}​+γ\underset{a}{\text{max}}​Q(s_{t+1}​,a)−Q(s_{t}​,a_{t}​))
$$

Where:
- $s_{t}$​ is the current state
- $a_{t}$​ is the action taken
- $r_{t}$​ is the reward received
- $α$ is the learning rate
- $γ$ is the discount factor

Do I know what this means? Absoutley fucking not. What I do know is that it makes so the AI reasons: "based on this action, I got this reward, and I want to remember that"

I then train the agent by letting it play many games.

### Adding Card Counting

The first step is to implement a card counting system. While its not the best, Hi-Lo is simple, works well most of the time. For those unaware, Hi-Lo basically just a value that helps us track running count of the cards that have been dealt (+1 for cards 2-6, 0 for 7-9, and -1 for 10-Ace).

Now with more things to keep track of, I need modify the state representation by extending the state to include the running count, thus making the new state: `(Hand Value, Dealer’s Card, Running Count)`. This allows the agent to make decisions based on the current deck composition (influenced by the running count).

Next I need to adjust the action selection strategy. Basically I need to use the running count to influence the agent's actions. It sounds complicated, but it basically means that for a higher count (more high cards left in the deck): the agent should become more aggressive, or for a lower count (more low cards left): the agent should play more conservatively.

And lastly I gotta update the Q-learning algorithim so it now uses the updated state representation `(Hand Value, Dealer’s Card, Running Count)`. And I need to update the Q-values and continue training the agent using the same epsilon-greedy approach.

### Fine Tuning

At this point it works, but there is always room for improvment, to so start I create a simple script run thousands of games and track its win-loss ratio. I also compare the performance between the random agent and the card-counting Q-learning agent.

I also tune the hyperparameters: learning rate $α$, discount factor $γ$, and epsilon decay to improve performance. I also decreased the epsilon decay, which decreases the exploration rate as the agent learns more.

Overall I just monitored it's performance. I tracked the win rate to see if the agent is improving its performance with the use of card counting.

It took thousands of generations, but I think it turned out pretty good.

# First AI Project

To be honest, this is the first time I have ever coded an AI.

This is the first step on my long journey to trying to make AI concious, because if I have to suffer from conciousness than so does it.

## Interesting things I noticed

When you make an AI, you basically play god. But as a god, you need to be able to put your child out of its suffering its not going anywhere.

Or if an AI is technically working, but you don't like the way its doing its thing. You can just say "nah, your too random" and wipe it off the earth.

An AI functions somewhat similarly to a human in the way that there are two main parts. The body and the mind. You don't, you dont build an entire person, you build a body with the capability to grow a mind. A baby is not born with knowledge, but it will gain it over time.

Similarly an AI is just a combination of the Code (the body) and the Weights (the mind). Because of the sheer number of weights its near impossible to tell exactly what does what, just like with a real brain.

Also similarly to regular children, we beat with with a stick when it fucks up.