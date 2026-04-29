# v6.2

## Definition of `struct xc_func_type` in v6.2

```C
struct xc_func_type{
  xc_func_info_type *info;             /* all the information concerning this functional */
  int nspin;                           /* XC_UNPOLARIZED or XC_POLARIZED  */

  int n_func_aux;                      /* how many auxiliary functions we need */
  struct xc_func_type **func_aux;      /* most GGAs are based on a LDA or other GGAs  */
  double *mix_coef;                    /* coefficients for the mixing */

  /**
     Parameters for range-separated hybrids
     cam_omega: the range separation constant
     cam_alpha: fraction of full Hartree-Fock exchange, used both for
                usual hybrids as well as range-separated ones
     cam_beta:  fraction of short-range only(!) exchange in
                range-separated hybrids

     N.B. Different conventions for alpha and beta can be found in
     literature. In the convention used in libxc, at short range the
     fraction of exact exchange is cam_alpha+cam_beta, while at long
     range it is cam_alpha.
  */
  double cam_omega, cam_alpha, cam_beta;

  double nlc_b;                /* Non-local correlation, b parameter */
  double nlc_C;                /* Non-local correlation, C parameter */

  xc_dimensions dim;           /* the dimensions of all input and output arrays */

  void *params;                /* this allows us to fix parameters in the functional */

  double dens_threshold;       /* functional is put to zero for spin-densities smaller than this */
  double zeta_threshold;       /* idem for the absolute value of zeta */
  double sigma_threshold;
  double tau_threshold;
};
```

# v7.0

## Definition of `struct xc_func_type` in v7.0

```C
struct xc_func_type{
  xc_func_info_type *info;             /* all the information concerning this functional */
  int nspin;                           /* XC_UNPOLARIZED or XC_POLARIZED  */

  int n_func_aux;                      /* how many auxiliary functions we need */
  struct xc_func_type **func_aux;      /* most GGAs are based on a LDA or other GGAs  */
  double *mix_coef;                    /* coefficients for the mixing */

  /**
     Parameters for range-separated hybrids
     cam_omega: the range separation constant
     cam_alpha: fraction of full Hartree-Fock exchange, used both for
                usual hybrids as well as range-separated ones
     cam_beta:  fraction of short-range only(!) exchange in
                range-separated hybrids

     N.B. Different conventions for alpha and beta can be found in
     literature. In the convention used in libxc, at short range the
     fraction of exact exchange is cam_alpha+cam_beta, while at long
     range it is cam_alpha.
  */
  double cam_omega, cam_alpha, cam_beta;

  double nlc_b;                /* Non-local correlation, b parameter */
  double nlc_C;                /* Non-local correlation, C parameter */

  xc_dimensions dim;           /* the dimensions of all input and output arrays */

  /* This is where the values of the external parameters are stored */
  double *ext_params;
  /* This is a placeholder for structs of parameters that are used in the Maple generated sources */
  void *params;

  double dens_threshold;       /* functional is put to zero for spin-densities smaller than this */
  double zeta_threshold;       /* idem for the absolute value of zeta */
  double sigma_threshold;
  double tau_threshold;
};
```

## Introduced in v7.0

xc_func_set_fhc_enforcement
xc_func_get_ext_params
xc_func_get_ext_params_name
xc_func_get_ext_params_value

# v7.1

## Introduced in v7.1

xc_func_info_get_default_flags
xc_func_info_set_default_flags
xc_func_init_flags
