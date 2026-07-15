# Concepts

## Brunt-Väisälä (buoyancy) frequency

The Brunt-Väisälä frequency $N$ is the buoyancy frequency, which represents whether the atmosphere is stably stratified or not (i.e., does the atmosphere resists vertical motion or not). It is computed as:

$$
N^2 = \frac{g}{T} \cdot \left( \frac{dT}{dz} + \frac{g}{c_p} \right),
$$

where:
1. $g$ is Mars's gravity $3.71 m/s^2$,
2. $T$ is the temperature at some altitude,
3. $dT/dz$ is the environmental lapse rate,
4. $g/c_p$ is the dry adiabatic lapse rate defined via gravity and the specific heat capacity at constant pressure (roughly $840 J/(kg\cdot K)$ on Mars).

Importantly, $dT/dz < 0$ (temperature decreases with altitude) and is less negative in winter than in summer. When $|dT/dz|$ approaches $g/c_p$, the atmosphere becomes less stable. Winter conditions (slower cooling with altitude) support stronger waves, because the atmosphere resists the vertical motion of air enough to induce oscillations. Therefore, we need $N^2 > 0$ to provide enough restoring force to induce the oscillations that form atmospheric gravity waves.

## Froude Number

The Froude number $F_r$ is a value that determines whether wind is expected to flow around a mountain or over it. It is important to check:

1. if $F_r < 1$, then flow is blocked and mountain waves will not form.
2. if $F_r \in [1, 3]$, then strong waves form.
3. if $F_r >> 1$, then weak waves will form leeward of the mountain ridge.

The Froude number is computed as follows:

$$
F_r = \frac{U}{N \cdot h},
$$

where:
1. $U$ is the wind speed interacting with the mountain ridge,
2. $N$ is the Brunt-Väisälä (buoyancy) frequency, and
3. $h$ is the height of the mountain ridge
