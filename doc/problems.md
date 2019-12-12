# Problems

## Dec 12, 2019

1. Check if implementation is correct
   1. `dj_da` computes
      $$J \bold{F}^{-T}$$
   2. `WeightIterator::Iterator::next` computes
      $$\frac{\partial w_{ijk}}{dx_i}, \frac{\partial w_{ijk}}{dx_j}, \frac{\partial w_{ijk}}{dx_k}$$
   3. `polar_svd_r` computes
      $$\bold{F} = \bold{U} \Sigma \bold{V}^T, \bold{R} = \bold{U} \bold{V}^T$$
   4. `fixed_corotated` computes
      $$\bold{P} = 2\mu (\bold{F} - \bold{R}) + \lambda (J - 1) J \bold{F}^{-T}$$
   5. `apply_elastic_force` constants if makes sense. e.g. what does `V_p^0` really means? Should I find
      another one?
2. Get rid of the `Nan`...
3. Is the current result enough?
4. Can we make $\mu$, $\lambda$ and other numbers per-particle? How does the particle insertion work then?
5. How to visualize this point cloud?

## Dec 11, 2019

1. Sometimes particles still go through boundary? Is there any ususally used parameters?
2. How does Elasticity work? How to add elasticity into force of each node?
3. What does `evolveF` do?