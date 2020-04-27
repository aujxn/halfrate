## Austen Nelson --- halfrate.rs

This program takes .wav files with a 16bit sample size and 48khz sample rate and
resamples them to 24khz sample rate. To prevent aliasing of frequencies in the
higher range of the source signal, a low half pass filter is applied to the input
using a finite impulse response filter. Coefficients for the filter were obtained
from scipy's signal module.

```
# Build filter coefficients for a half-band filter.
# Bart Massey 2020

from scipy import signal

# Build a Kaiser window filter with "optimal" length and
# "beta" for -40 dB of passband and stopband ripple and a
# 0.05 transition bandwidth. Prescale the coefficients to
# preserve the input amplitude.
nopt, bopt = signal.kaiserord(-40, 0.05)
subband = signal.firwin(nopt, 0.45, window=('kaiser', bopt), scale=True)

# Display the coefficients.
for s in subband:
    print(s)
```

Convolving these coefficients on the source signal with a stride of 2
results in a signal with half the sample rate and limited aliasing of
the upper frequencies.
