//! Libxc functional IDs for API version 7.1.
//!
//! This file is generated automatically from `xc_funcs_v7.1.h`.
//! Do not edit manually.

#![allow(non_camel_case_types)]
#![allow(clippy::enum_clike_unportable_variant)]

use serde::{Deserialize, Serialize};

/// Libxc functional identifier numbers.
///
/// Each variant corresponds to a `XC_*` define in the libxc headers.
/// The numeric values match the C library exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum XcFuncId {
    /// Slater exchange
    LDA_X = 1,
    /// Wigner
    LDA_C_WIGNER = 2,
    /// Random Phase Approximation (RPA)
    LDA_C_RPA = 3,
    /// Hedin & Lundqvist
    LDA_C_HL = 4,
    /// Gunnarsson & Lundqvist
    LDA_C_GL = 5,
    /// Slater's Xalpha
    LDA_C_XALPHA = 6,
    /// Vosko, Wilk & Nusair (VWN5)
    LDA_C_VWN = 7,
    /// Vosko, Wilk & Nusair (VWN5_RPA)
    LDA_C_VWN_RPA = 8,
    /// Perdew & Zunger
    LDA_C_PZ = 9,
    /// Perdew & Zunger (Modified)
    LDA_C_PZ_MOD = 10,
    /// Ortiz & Ballone (PZ parametrization)
    LDA_C_OB_PZ = 11,
    /// Perdew & Wang
    LDA_C_PW = 12,
    /// Perdew & Wang (modified)
    LDA_C_PW_MOD = 13,
    /// Ortiz & Ballone (PW parametrization)
    LDA_C_OB_PW = 14,
    /// AMGB (for 2D systems)
    LDA_C_2D_AMGB = 15,
    /// PRM (for 2D systems)
    LDA_C_2D_PRM = 16,
    /// von Barth & Hedin
    LDA_C_VBH = 17,
    /// Casula, Sorella & Senatore
    LDA_C_1D_CSS = 18,
    /// Slater exchange
    LDA_X_2D = 19,
    /// Teter 93
    LDA_XC_TETER93 = 20,
    /// Exchange in 1D for an soft-Coulomb interaction
    LDA_X_1D_SOFT = 21,
    /// Modified LSD (version 1) of Proynov and Salahub
    LDA_C_ML1 = 22,
    /// Modified LSD (version 2) of Proynov and Salahub
    LDA_C_ML2 = 23,
    /// Gombas
    LDA_C_GOMBAS = 24,
    /// Perdew & Wang (fit to the RPA energy)
    LDA_C_PW_RPA = 25,
    /// P-F Loos correlation LDA
    LDA_C_1D_LOOS = 26,
    /// Ragot-Cortona
    LDA_C_RC04 = 27,
    /// Vosko, Wilk & Nusair (VWN1)
    LDA_C_VWN_1 = 28,
    /// Vosko, Wilk & Nusair (VWN2)
    LDA_C_VWN_2 = 29,
    /// Vosko, Wilk & Nusair (VWN3)
    LDA_C_VWN_3 = 30,
    /// Vosko, Wilk & Nusair (VWN4)
    LDA_C_VWN_4 = 31,
    /// Minnesota GAM exhange functional
    GGA_X_GAM = 32,
    /// Minnesota GAM correlation functional
    GGA_C_GAM = 33,
    /// HCTH-A
    GGA_X_HCTH_A = 34,
    /// Engel and Vosko
    GGA_X_EV93 = 35,
    /// Dispersionless Density Functional
    HYB_MGGA_X_DLDF = 36,
    /// Dispersionless Density Functional
    MGGA_C_DLDF = 37,
    /// Burke, Cancio, Gould, and Pittalis
    GGA_X_BCGP = 38,
    /// acGGA, asymptotically corrected GGA correlation
    GGA_C_ACGGA = 39,
    /// lambda_OC2(N) version of PBE
    GGA_X_LAMBDA_OC2_N = 40,
    /// Revised Becke 86 with modified gradient correction
    GGA_X_B86_R = 41,
    /// Zhao, Levy & Parr, Eq. (21)
    MGGA_XC_ZLP = 42,
    /// Zhao, Levy & Parr, Eq. (20)
    LDA_XC_ZLP = 43,
    /// lambda_CH(N) version of PBE
    GGA_X_LAMBDA_CH_N = 44,
    /// lambda_LO(N) version of PBE
    GGA_X_LAMBDA_LO_N = 45,
    /// HJS screened exchange B88 corrected version
    GGA_X_HJS_B88_V2 = 46,
    /// Chiodo et al
    GGA_C_Q2D = 47,
    /// Chiodo et al
    GGA_X_Q2D = 48,
    /// Reparametrized PBE by del Campo, Gazquez, Trickey & Vela
    GGA_X_PBE_MOL = 49,
    /// Thomas-Fermi kinetic energy
    LDA_K_TF = 50,
    /// Lee and Parr Gaussian ansatz for the kinetic energy
    LDA_K_LP = 51,
    /// Thomas-Fermi plus von Weiszaecker correction
    GGA_K_TFVW = 52,
    /// interpolated version of revAPBE
    GGA_K_REVAPBEINT = 53,
    /// interpolated version of APBE
    GGA_K_APBEINT = 54,
    /// revised APBE
    GGA_K_REVAPBE = 55,
    /// Armiento & Kuemmel 2013
    GGA_X_AK13 = 56,
    /// Meyer,  Wang, and Young
    GGA_K_MEYER = 57,
    /// Berland and Hyldgaard
    GGA_X_LV_RPW86 = 58,
    /// PBE revised by Tognetti et al
    GGA_X_PBE_TCA = 59,
    /// PBE for hybrid interfaces
    GGA_X_PBEINT = 60,
    /// spin-dependent gradient correction to PBEint
    GGA_C_ZPBEINT = 61,
    /// PBE for hybrid interfaces
    GGA_C_PBEINT = 62,
    /// spin-dependent gradient correction to PBEsol
    GGA_C_ZPBESOL = 63,
    /// oTPSS-D functional of Goerigk and Grimme
    MGGA_XC_OTPSS_D = 64,
    /// oPBE-D functional of Goerigk and Grimme
    GGA_XC_OPBE_D = 65,
    /// oPWLYP-D functional of Goerigk and Grimme
    GGA_XC_OPWLYP_D = 66,
    /// oBLYP-D functional of Goerigk and Grimme
    GGA_XC_OBLYP_D = 67,
    /// VMT{8,4} with constraint satisfaction with mu = mu_GE
    GGA_X_VMT84_GE = 68,
    /// VMT{8,4} with constraint satisfaction with mu = mu_PBE
    GGA_X_VMT84_PBE = 69,
    /// Vela, Medel, and Trickey with mu = mu_GE
    GGA_X_VMT_GE = 70,
    /// Vela, Medel, and Trickey with mu = mu_PBE
    GGA_X_VMT_PBE = 71,
    /// Colle and Salvetti
    MGGA_C_CS = 72,
    /// Minnesota MN12-SX correlation functional
    MGGA_C_MN12_SX = 73,
    /// Minnesota MN12-L correlation functional
    MGGA_C_MN12_L = 74,
    /// Minnesota M11-L correlation functional
    MGGA_C_M11_L = 75,
    /// Minnesota M11 correlation functional
    MGGA_C_M11 = 76,
    /// Minnesota M08-SO correlation functional
    MGGA_C_M08_SO = 77,
    /// Minnesota M08 correlation functional
    MGGA_C_M08_HX = 78,
    /// Minnesota N12-SX correlation functional
    GGA_C_N12_SX = 79,
    /// Minnesota N12 correlation functional
    GGA_C_N12 = 80,
    /// Minnesota N12-SX exchange functional
    HYB_GGA_X_N12_SX = 81,
    /// Minnesota N12 exchange functional
    GGA_X_N12 = 82,
    /// regularized TPSS correlation
    GGA_C_REGTPSS = 83,
    /// one-parameter progressive functional (Xalpha version)
    GGA_C_OP_XALPHA = 84,
    /// one-parameter progressive functional (G96 version)
    GGA_C_OP_G96 = 85,
    /// one-parameter progressive functional (PBE version)
    GGA_C_OP_PBE = 86,
    /// one-parameter progressive functional (B88 version)
    GGA_C_OP_B88 = 87,
    /// Filatov & Thiel correlation
    GGA_C_FT97 = 88,
    /// PBE correlation to be used with the SSB exchange
    GGA_C_SPBE = 89,
    /// Swart, Sola and Bickelhaupt correction to PBE
    GGA_X_SSB_SW = 90,
    /// Swart, Sola and Bickelhaupt
    GGA_X_SSB = 91,
    /// Swart, Sola and Bickelhaupt dispersion
    GGA_X_SSB_D = 92,
    /// HCTH/407+
    GGA_XC_HCTH_407P = 93,
    /// HCTH p=7/6
    GGA_XC_HCTH_P76 = 94,
    /// HCTH p=1/4
    GGA_XC_HCTH_P14 = 95,
    /// Becke 97 GGA-1
    GGA_XC_B97_GGA1 = 96,
    /// HCTH-A
    GGA_C_HCTH_A = 97,
    /// BPCCAC (GRAC for the energy)
    GGA_X_BPCCAC = 98,
    /// Tognetti, Cortona, Adamo (revised)
    GGA_C_REVTCA = 99,
    /// Tognetti, Cortona, Adamo
    GGA_C_TCA = 100,
    /// Perdew, Burke & Ernzerhof
    GGA_X_PBE = 101,
    /// Revised PBE from Zhang & Yang
    GGA_X_PBE_R = 102,
    /// Becke 86
    GGA_X_B86 = 103,
    /// Becke's original half-and-half functional: 50% HF and 50% LDA xc
    HYB_LDA_XC_B93 = 104,
    /// Becke 86 with modified gradient correction
    GGA_X_B86_MGC = 105,
    /// Becke 88
    GGA_X_B88 = 106,
    /// Gill 96
    GGA_X_G96 = 107,
    /// Perdew & Wang 86
    GGA_X_PW86 = 108,
    /// Perdew & Wang 91
    GGA_X_PW91 = 109,
    /// Handy & Cohen OPTX 01
    GGA_X_OPTX = 110,
    /// dePristo & Kress 87 version R1
    GGA_X_DK87_R1 = 111,
    /// dePristo & Kress 87 version R2
    GGA_X_DK87_R2 = 112,
    /// Lacks & Gordon 93
    GGA_X_LG93 = 113,
    /// Filatov & Thiel 97 (version A)
    GGA_X_FT97_A = 114,
    /// Filatov & Thiel 97 (version B)
    GGA_X_FT97_B = 115,
    /// Perdew, Burke & Ernzerhof SOL
    GGA_X_PBE_SOL = 116,
    /// Hammer, Hansen, and Norskov
    GGA_X_RPBE = 117,
    /// Wu & Cohen
    GGA_X_WC = 118,
    /// mPW91 of Adamo & Barone
    GGA_X_MPW91 = 119,
    /// Armiento & Mattsson 05
    GGA_X_AM05 = 120,
    /// Madsen 07
    GGA_X_PBEA = 121,
    /// Adamo & Barone modification to PBE
    GGA_X_MPBE = 122,
    /// Extended PBE by Xu & Goddard III
    GGA_X_XPBE = 123,
    /// Becke 86 with modified gradient correction for 2D
    GGA_X_2D_B86_MGC = 124,
    /// Bayesian best fit for the enhancement factor
    GGA_X_BAYESIAN = 125,
    /// Reparametrized PBE by Pedroza, Silva & Capelle
    GGA_X_PBE_JSJR = 126,
    /// Becke 88 in 2D
    GGA_X_2D_B88 = 127,
    /// Becke 86 in 2D
    GGA_X_2D_B86 = 128,
    /// Perdew, Burke & Ernzerhof in 2D
    GGA_X_2D_PBE = 129,
    /// Perdew, Burke & Ernzerhof
    GGA_C_PBE = 130,
    /// Lee, Yang & Parr
    GGA_C_LYP = 131,
    /// Perdew 86
    GGA_C_P86 = 132,
    /// Perdew, Burke & Ernzerhof SOL
    GGA_C_PBE_SOL = 133,
    /// Perdew & Wang 91
    GGA_C_PW91 = 134,
    /// Armiento & Mattsson 05
    GGA_C_AM05 = 135,
    /// Extended PBE by Xu & Goddard III
    GGA_C_XPBE = 136,
    /// Langreth & Mehl
    GGA_C_LM = 137,
    /// Reparametrized PBE by Pedroza, Silva & Capelle
    GGA_C_PBE_JRGX = 138,
    /// opt-Becke 88 for vdW
    GGA_X_OPTB88_VDW = 139,
    /// Reparametrized PBE for vdW
    GGA_X_PBEK1_VDW = 140,
    /// Reparametrized PBE for vdW
    GGA_X_OPTPBE_VDW = 141,
    /// Regularized PBE
    GGA_X_RGE2 = 142,
    /// Regularized PBE
    GGA_C_RGE2 = 143,
    /// Refitted Perdew & Wang 86
    GGA_X_RPW86 = 144,
    /// Exchange part of Keal and Tozer version 1
    GGA_X_KT1 = 145,
    /// Keal and Tozer, version 2
    GGA_XC_KT2 = 146,
    /// Wilson & Levy
    GGA_C_WL = 147,
    /// Wilson & Ivanov
    GGA_C_WI = 148,
    /// Modified Becke 88 for proton transfer
    GGA_X_MB88 = 149,
    /// Second-order generalized gradient approximation
    GGA_X_SOGGA = 150,
    /// Second-order generalized gradient approximation 2011
    GGA_X_SOGGA11 = 151,
    /// Second-order generalized gradient approximation 2011
    GGA_C_SOGGA11 = 152,
    /// Wilson & Ivanov initial version
    GGA_C_WI0 = 153,
    /// Tozer and Handy v. 1
    GGA_XC_TH1 = 154,
    /// Tozer and Handy v. 2
    GGA_XC_TH2 = 155,
    /// Tozer and Handy v. 3
    GGA_XC_TH3 = 156,
    /// Tozer and Handy v. 4
    GGA_XC_TH4 = 157,
    /// C09x to be used with the VdW of Rutgers-Chalmers
    GGA_X_C09X = 158,
    /// To be used with HYB_GGA_X_SOGGA11_X
    GGA_C_SOGGA11_X = 159,
    /// van Leeuwen & Baerends
    GGA_X_LB = 160,
    /// HCTH/93
    GGA_XC_HCTH_93 = 161,
    /// HCTH/120
    GGA_XC_HCTH_120 = 162,
    /// HCTH/147
    GGA_XC_HCTH_147 = 163,
    /// HCTH/407
    GGA_XC_HCTH_407 = 164,
    /// EDF1
    GGA_XC_EDF1 = 165,
    /// XLYP
    GGA_XC_XLYP = 166,
    /// Keal and Tozer, version 1
    GGA_XC_KT1 = 167,
    /// lsPBE, a PW91-like modification of PBE exchange
    GGA_X_LSPBE = 168,
    /// lsRPBE, a PW91-like modification of RPBE
    GGA_X_LSRPBE = 169,
    /// Becke 97-D
    GGA_XC_B97_D = 170,
    /// Becke 86 reoptimized for use with vdW functional of Dion et al
    GGA_X_OPTB86B_VDW = 171,
    /// Revised Minnesota M11 correlation functional
    MGGA_C_REVM11 = 172,
    /// PBE1W
    GGA_XC_PBE1W = 173,
    /// mPWLYP1w
    GGA_XC_MPWLYP1W = 174,
    /// PBELYP1W
    GGA_XC_PBELYP1W = 175,
    /// acGGA+, asymptotically corrected GGA correlation+
    GGA_C_ACGGAP = 176,
    /// LDA hybrid exchange (LDA0)
    HYB_LDA_XC_LDA0 = 177,
    /// CAM version of LDA0
    HYB_LDA_XC_CAM_LDA0 = 178,
    /// Becke 88 reoptimized with the 6-311G** basis set
    GGA_X_B88_6311G = 179,
    /// Nearly correct asymptotic potential
    GGA_X_NCAP = 180,
    /// NCAP exchange + P86 correlation
    GGA_XC_NCAP = 181,
    /// van Leeuwen & Baerends modified
    GGA_X_LBM = 182,
    /// Exchange form based on Ou-Yang and Levy v.2
    GGA_X_OL2 = 183,
    /// mu fixed from the semiclassical neutral atom
    GGA_X_APBE = 184,
    /// mu fixed from the semiclassical neutral atom
    GGA_K_APBE = 185,
    /// mu fixed from the semiclassical neutral atom
    GGA_C_APBE = 186,
    /// Tran and Wesolowski set 1 (Table II)
    GGA_K_TW1 = 187,
    /// Tran and Wesolowski set 2 (Table II)
    GGA_K_TW2 = 188,
    /// Tran and Wesolowski set 3 (Table II)
    GGA_K_TW3 = 189,
    /// Tran and Wesolowski set 4 (Table II)
    GGA_K_TW4 = 190,
    /// Haas, Tran, Blaha, and Schwarz
    GGA_X_HTBS = 191,
    /// Constantin et al based on the Airy gas
    GGA_X_AIRY = 192,
    /// Local Airy Gas
    GGA_X_LAG = 193,
    /// Functional for organometallic chemistry
    GGA_XC_MOHLYP = 194,
    /// Functional for barrier heights
    GGA_XC_MOHLYP2 = 195,
    /// Tozer and Handy v. FL
    LDA_XC_TH_FL = 196,
    /// Tozer and Handy v. FC
    GGA_XC_TH_FC = 197,
    /// Tozer and Handy v. FCFO
    GGA_XC_TH_FCFO = 198,
    /// Tozer and Handy v. FCO
    GGA_XC_TH_FCO = 199,
    /// Optimized correlation functional of Cohen and Handy
    GGA_C_OPTC = 200,
    /// Local tau approximation
    MGGA_X_LTA = 201,
    /// Tao, Perdew, Staroverov & Scuseria
    MGGA_X_TPSS = 202,
    /// Minnesota M06-L exchange functional
    MGGA_X_M06_L = 203,
    /// GVT4 (X part of VSXC)
    MGGA_X_GVT4 = 204,
    /// tau-HCTH from Boese and Handy
    MGGA_X_TAU_HCTH = 205,
    /// Becke-Roussel 89, gamma = 0.8
    MGGA_X_BR89 = 206,
    /// Becke & Johnson 06
    MGGA_X_BJ06 = 207,
    /// Tran & Blaha 09
    MGGA_X_TB09 = 208,
    /// Rasanen, Pittalis & Proetto 09
    MGGA_X_RPP09 = 209,
    /// Pittalis-Rasanen-Helbig-Gross 2007
    MGGA_X_2D_PRHG07 = 210,
    /// PRHG07 with Pittalis-Rasanen-Proetto 2010 correction
    MGGA_X_2D_PRHG07_PRP10 = 211,
    /// revised Tao, Perdew, Staroverov & Scuseria
    MGGA_X_REVTPSS = 212,
    /// Perdew, Kurth, Zupan, and Blaha
    MGGA_X_PKZB = 213,
    /// Becke-Roussel 89, gamma = 1.0
    MGGA_X_BR89_1 = 214,
    /// Engel, Chevary, Macdonald and Vosko
    GGA_X_ECMV92 = 215,
    /// Perdew, Burke & Ernzerhof based on VWN correlation
    GGA_C_PBE_VWN = 216,
    /// Perdew 86 with more accurate value for ftilde
    GGA_C_P86_FT = 217,
    /// RATIONAL$^{p}$ by Lehtomaki and Lopez-Acevedo (by default $p=3/2$,
    /// $C_{2}=0.7687$)
    GGA_K_RATIONAL_P = 218,
    /// PG1 (Pauli-Gaussian) functional by Constantin, Fabiano, and Della Sala
    GGA_K_PG1 = 219,
    /// PGSL025 (Pauli-Gaussian) functional by Constantin, Fabiano, and Della
    /// Sala
    MGGA_K_PGSL025 = 220,
    /// MS exchange of Sun, Xiao, and Ruzsinszky
    MGGA_X_MS0 = 221,
    /// MS1 exchange of Sun, et al
    MGGA_X_MS1 = 222,
    /// MS2 exchange of Sun, et al
    MGGA_X_MS2 = 223,
    /// MS2 hybrid exchange of Sun, et al
    HYB_MGGA_X_MS2H = 224,
    /// Tsuneda and Hirao
    MGGA_X_TH = 225,
    /// Minnesota M11-L exchange functional
    MGGA_X_M11_L = 226,
    /// Minnesota MN12-L exchange functional
    MGGA_X_MN12_L = 227,
    /// MS2 exchange of Sun, et al with revised value for c
    MGGA_X_MS2_REV = 228,
    /// Cancio and Chou 2006
    MGGA_XC_CC06 = 229,
    /// Ghosh-Parr 1986 meta-GGA exchange, later reinvestigated by Manby and
    /// Knowles
    MGGA_X_GP86 = 230,
    /// Tao, Perdew, Staroverov & Scuseria
    MGGA_C_TPSS = 231,
    /// VSXC (correlation part)
    MGGA_C_VSXC = 232,
    /// Minnesota M06-L correlation functional
    MGGA_C_M06_L = 233,
    /// Minnesota M06-HF correlation functional
    MGGA_C_M06_HF = 234,
    /// Minnesota M06 correlation functional
    MGGA_C_M06 = 235,
    /// Minnesota M06-2X correlation functional
    MGGA_C_M06_2X = 236,
    /// Minnesota M05 correlation functional
    MGGA_C_M05 = 237,
    /// Minnesota M05-2X correlation functional
    MGGA_C_M05_2X = 238,
    /// Perdew, Kurth, Zupan, and Blaha
    MGGA_C_PKZB = 239,
    /// Becke correlation 95
    MGGA_C_BC95 = 240,
    /// revised TPSS correlation
    MGGA_C_REVTPSS = 241,
    /// TPSSLYP1W
    MGGA_XC_TPSSLYP1W = 242,
    /// Exchange for accurate virtual orbital energies (v. B)
    MGGA_X_MK00B = 243,
    /// functional with balanced localization
    MGGA_X_BLOC = 244,
    /// Modified Tao, Perdew, Staroverov & Scuseria
    MGGA_X_MODTPSS = 245,
    /// Semilocal dynamical correlation
    GGA_C_PBELOC = 246,
    /// Semilocal dynamical correlation
    MGGA_C_TPSSLOC = 247,
    /// Minnesota MN12-SX hybrid exchange functional
    HYB_MGGA_X_MN12_SX = 248,
    /// mBEEF exchange
    MGGA_X_MBEEF = 249,
    /// mBEEF-vdW exchange
    MGGA_X_MBEEFVDW = 250,
    /// Tao and Mo 2016 correlation
    MGGA_C_TM = 251,
    /// Perdew 86 based on VWN5 correlation
    GGA_C_P86VWN = 252,
    /// Perdew 86 based on VWN5 correlation, with more accurate value for ftilde
    GGA_C_P86VWN_FT = 253,
    /// B97M-V exchange-correlation functional
    MGGA_XC_B97M_V = 254,
    /// Vydrov and Van Voorhis
    GGA_XC_VV10 = 255,
    /// Jemmer-Knowles meta-GGA exchange
    MGGA_X_JK = 256,
    /// MVS exchange of Sun, Perdew, and Ruzsinszky
    MGGA_X_MVS = 257,
    /// PBE for formation energies
    GGA_C_PBEFE = 258,
    /// Karasiev, Sjostrom, Dufty & Trickey
    LDA_XC_KSDT = 259,
    /// Minnesota MN15-L exchange functional
    MGGA_X_MN15_L = 260,
    /// Minnesota MN15-L correlation functional
    MGGA_C_MN15_L = 261,
    /// one-parameter progressive functional (PW91 version)
    GGA_C_OP_PW91 = 262,
    /// SCAN exchange of Sun, Ruzsinszky, and Perdew
    MGGA_X_SCAN = 263,
    /// SCAN hybrid exchange (SCAN0)
    HYB_MGGA_X_SCAN0 = 264,
    /// PBE for formation energies
    GGA_X_PBEFE = 265,
    /// version of B97 by Cohen and Handy
    HYB_GGA_XC_B97_1P = 266,
    /// SCAN correlation of Sun, Ruzsinszky, and Perdew
    MGGA_C_SCAN = 267,
    /// Minnesota MN15 hybrid exchange functional
    HYB_MGGA_X_MN15 = 268,
    /// Minnesota MN15 correlation functional
    MGGA_C_MN15 = 269,
    /// Correct Asymptotic Potential
    GGA_X_CAP = 270,
    /// Non-empirical (excogitated) B88 functional of Becke and Elliott
    GGA_X_EB88 = 271,
    /// Reparametrized PBE by del Campo, Gazquez, Trickey & Vela
    GGA_C_PBE_MOL = 272,
    /// PBEmol0
    HYB_GGA_XC_PBE_MOL0 = 273,
    /// PBEsol0
    HYB_GGA_XC_PBE_SOL0 = 274,
    /// PBEbeta0
    HYB_GGA_XC_PBEB0 = 275,
    /// PBEmolbeta0
    HYB_GGA_XC_PBE_MOLB0 = 276,
    /// gamma-TFvW form by Acharya et al [$g = 1 - 1.513/N^{0.35}]$
    GGA_K_ABSP3 = 277,
    /// gamma-TFvW form by Acharya et al [$g = l = 1/(1 + 1.332/N^{1/3})$]
    GGA_K_ABSP4 = 278,
    /// Boese-Martin for kinetics
    HYB_MGGA_X_BMK = 279,
    /// Boese-Martin correlation for kinetics
    GGA_C_BMK = 280,
    /// correlation part of tau-hcth
    GGA_C_TAU_HCTH = 281,
    /// Hybrid version of tau-HCTH
    HYB_MGGA_X_TAU_HCTH = 282,
    /// correlation part of hyb-tau-hcth
    GGA_C_HYB_TAU_HCTH = 283,
    /// Becke 2000
    MGGA_X_B00 = 284,
    /// BEEF-vdW exchange
    GGA_X_BEEFVDW = 285,
    /// BEEF-vdW exchange-correlation
    GGA_XC_BEEFVDW = 286,
    /// Chachiyo simple 2 parameter correlation
    LDA_C_CHACHIYO = 287,
    /// high local exchange 2017
    MGGA_XC_HLE17 = 288,
    /// Liu-Parr correlation
    LDA_C_LP96 = 289,
    /// PBE50
    HYB_GGA_XC_PBE50 = 290,
    /// Gradient-regulated connection-based correction for the PBE exchange
    GGA_X_PBETRANS = 291,
    /// SCAN + rVV10 correlation
    MGGA_C_SCAN_RVV10 = 292,
    /// Minnesota revM06-L exchange functional
    MGGA_X_REVM06_L = 293,
    /// Minnesota revM06-L correlation functional
    MGGA_C_REVM06_L = 294,
    /// Minnesota M08-HX hybrid exchange functional
    HYB_MGGA_X_M08_HX = 295,
    /// Minnesota M08-SO hybrid exchange functional
    HYB_MGGA_X_M08_SO = 296,
    /// Minnesota M11 hybrid exchange functional
    HYB_MGGA_X_M11 = 297,
    /// Chachiyo exchange
    GGA_X_CHACHIYO = 298,
    /// TPSS for surface adsorption
    MGGA_X_RTPSS = 299,
    /// MS2beta exchange of Furness and Sun
    MGGA_X_MS2B = 300,
    /// MS2beta* exchange of Furness and Sun
    MGGA_X_MS2BS = 301,
    /// MVSbeta exchange by Furness and Sun
    MGGA_X_MVSB = 302,
    /// MVSbeta* exchange by Furness and Sun
    MGGA_X_MVSBS = 303,
    /// Revised Minnesota M11 hybrid exchange functional
    HYB_MGGA_X_REVM11 = 304,
    /// Revised Minnesota M06 hybrid exchange functional
    HYB_MGGA_X_REVM06 = 305,
    /// Revised Minnesota M06 correlation functional
    MGGA_C_REVM06 = 306,
    /// Chachiyo simple 2 parameter correlation with modified spin scaling
    LDA_C_CHACHIYO_MOD = 307,
    /// Karasiev reparameterization of Chachiyo
    LDA_C_KARASIEV_MOD = 308,
    /// Chachiyo simple GGA correlation
    GGA_C_CHACHIYO = 309,
    /// Minnesota M06-SX short-range hybrid exchange functional
    HYB_MGGA_X_M06_SX = 310,
    /// Minnesota M06-SX correlation functional
    MGGA_C_M06_SX = 311,
    /// Revised Swart, Sola and Bickelhaupt dispersion
    GGA_X_REVSSB_D = 312,
    /// ccDF: coupled-cluster motivated density functional
    GGA_C_CCDF = 313,
    /// HF + LYP correlation
    HYB_GGA_XC_HFLYP = 314,
    /// B3P86, NWChem version
    HYB_GGA_XC_B3P86_NWCHEM = 315,
    /// PW91, alternate version with more digits
    GGA_X_PW91_MOD = 316,
    /// Xie, Wu, and Zhao interpolation ansatz without fitting parameters
    LDA_C_W20 = 317,
    /// Corrected KSDT by Karasiev, Dufty and Trickey
    LDA_XC_CORRKSDT = 318,
    /// Filatov and Thiel 1998 meta-GGA exchange
    MGGA_X_FT98 = 319,
    /// Perdew, Burke & Ernzerhof with less precise value for beta
    GGA_X_PBE_MOD = 320,
    /// Perdew, Burke & Ernzerhof with parameter values used in Gaussian
    GGA_X_PBE_GAUSSIAN = 321,
    /// Perdew, Burke & Ernzerhof with parameters from Gaussian
    GGA_C_PBE_GAUSSIAN = 322,
    /// Tao, Perdew, Staroverov & Scuseria with parameters from Gaussian
    MGGA_C_TPSS_GAUSSIAN = 323,
    /// Nearly correct asymptotic potential revised
    GGA_X_NCAPR = 324,
    /// relPBE0 a.k.a. relPBE: PBE0 refitted for actinide compounds
    HYB_GGA_XC_RELPBE0 = 325,
    /// Exact exchange-like exchange of Aschebrock et al
    MGGA_X_EEL = 326,
    /// Becke 97-3c by Grimme et. al.
    GGA_XC_B97_3C = 327,
    /// epc17(-1): electron-proton correlation 2017
    LDA_C_EPC17 = 328,
    /// epc17-2: electron-proton correlation 2017 for proton affinities
    LDA_C_EPC17_2 = 329,
    /// epc18-1: electron-proton correlation 2018
    LDA_C_EPC18_1 = 330,
    /// epc18-2: electron-proton correlation 2018 for proton affinities
    LDA_C_EPC18_2 = 331,
    /// dispersionless-optimized B97
    GGA_XC_DLB97 = 332,
    /// Modified SCAN (mSCAN) exchange of Desmarais, Erba, Vignale, and Pittalis
    MGGA_X_MSCAN = 333,
    /// Modified SCAN (mSCAN) correlation of Desmarais, Erba, Vignale, and
    /// Pittalis
    MGGA_C_MSCAN = 334,
    /// PBE reparametrization (version 1) for band gaps
    GGA_X_T_PBE1 = 335,
    /// PBE reparametrization (version 2) for band gaps
    GGA_X_T_PBE2 = 336,
    /// SLOC reparametrization for band gaps
    LDA_X_T_SLOC = 337,
    /// Exchange part of type-I band gap functional by Bhattacharjee, Koshi and
    /// Lee
    GGA_X_BKL1 = 338,
    /// Exchange part of type-II band gap functional by Bhattacharjee, Koshi and
    /// Lee
    GGA_X_BKL2 = 339,
    /// Minnesota CF22D hybrid exchange functional
    HYB_MGGA_X_CF22D = 340,
    /// Minnesota CF22D correlation functional
    MGGA_C_CF22D = 341,
    /// Lebeda-Aschebrock-Kummel meta-GGA exchange
    MGGA_X_LAK = 342,
    /// Correlation part of type-I band gap functional by Bhattacharjee, Koshi
    /// and Lee
    GGA_C_BKL1 = 343,
    /// Correlation part of type-II band gap functional by Bhattacharjee, Koshi
    /// and Lee
    GGA_C_BKL2 = 344,
    /// Lebeda-Aschebrock-Kummel meta-GGA correlation
    MGGA_C_LAK = 345,
    /// Becke 88 reoptimized by Chakraborty et al for use with vdW functional
    GGA_X_DF3_OPT1 = 346,
    /// Becke 86 reoptimized by Chakraborty et al for use with vdW functional
    GGA_X_DF3_OPT2 = 347,
    /// CAM-B3LYP retuned for core electron ionization energies
    HYB_GGA_XC_CQTP25 = 385,
    /// opB3LYP: B3LYP reoptimized in 6-311++G(2d,2p) basis set
    HYB_GGA_XC_OPB3LYP = 386,
    /// Self-interaction corrected correlation functional by Schmidt et al
    MGGA_C_CC = 387,
    /// Iso-orbital corrected LDA correlation by Lebeda et al
    MGGA_C_CCALDA = 388,
    /// BR3P86 hybrid meta-GGA from Neumann and Handy
    HYB_MGGA_XC_BR3P86 = 389,
    /// CASE21: Constrained And Smoothed semi-Empirical 2021 functional
    HYB_GGA_XC_CASE21 = 390,
    /// Revised regTM correlation by Jana et al
    MGGA_C_RREGTM = 391,
    /// PBE-2X: PBE0 with 56% exact exchange
    HYB_GGA_XC_PBE_2X = 392,
    /// PBE38: PBE0 with 3/8 = 37.5% exact exchange
    HYB_GGA_XC_PBE38 = 393,
    /// B3LYP with VWN functional 3 instead of RPA
    HYB_GGA_XC_B3LYP3 = 394,
    /// CAM-O3LYP
    HYB_GGA_XC_CAM_O3LYP = 395,
    /// TPSS0 with 25% exact exchange
    HYB_MGGA_XC_TPSS0 = 396,
    /// Becke 1994 meta-GGA correlation
    MGGA_C_B94 = 397,
    /// Becke 1994 hybrid meta-GGA
    HYB_MGGA_XC_B94_HYB = 398,
    /// wB97X-D3 range-separated functional
    HYB_GGA_XC_WB97X_D3 = 399,
    /// LC version of BLYP
    HYB_GGA_XC_LC_BLYP = 400,
    /// The original (ACM, B3PW91) hybrid of Becke
    HYB_GGA_XC_B3PW91 = 401,
    /// B3LYP
    HYB_GGA_XC_B3LYP = 402,
    /// B3P86
    HYB_GGA_XC_B3P86 = 403,
    /// O3LYP
    HYB_GGA_XC_O3LYP = 404,
    /// mPW1K
    HYB_GGA_XC_MPW1K = 405,
    /// PBEH (PBE0)
    HYB_GGA_XC_PBEH = 406,
    /// Becke 97
    HYB_GGA_XC_B97 = 407,
    /// Becke 97-1
    HYB_GGA_XC_B97_1 = 408,
    /// APF hybrid functional
    HYB_GGA_XC_APF = 409,
    /// Becke 97-2
    HYB_GGA_XC_B97_2 = 410,
    /// X3LYP
    HYB_GGA_XC_X3LYP = 411,
    /// B1WC
    HYB_GGA_XC_B1WC = 412,
    /// Boese-Martin for Kinetics
    HYB_GGA_XC_B97_K = 413,
    /// Becke 97-3
    HYB_GGA_XC_B97_3 = 414,
    /// MPW3PW of Adamo & Barone
    HYB_GGA_XC_MPW3PW = 415,
    /// B1LYP
    HYB_GGA_XC_B1LYP = 416,
    /// B1PW91
    HYB_GGA_XC_B1PW91 = 417,
    /// mPW1PW
    HYB_GGA_XC_MPW1PW = 418,
    /// MPW3LYP
    HYB_GGA_XC_MPW3LYP = 419,
    /// SB98 (1a)
    HYB_GGA_XC_SB98_1A = 420,
    /// SB98 (1b)
    HYB_GGA_XC_SB98_1B = 421,
    /// SB98 (1c)
    HYB_GGA_XC_SB98_1C = 422,
    /// SB98 (2a)
    HYB_GGA_XC_SB98_2A = 423,
    /// SB98 (2b)
    HYB_GGA_XC_SB98_2B = 424,
    /// SB98 (2c)
    HYB_GGA_XC_SB98_2C = 425,
    /// Hybrid based on SOGGA11 form
    HYB_GGA_X_SOGGA11_X = 426,
    /// HSE03
    HYB_GGA_XC_HSE03 = 427,
    /// HSE06
    HYB_GGA_XC_HSE06 = 428,
    /// HJS hybrid screened exchange PBE version
    HYB_GGA_XC_HJS_PBE = 429,
    /// HJS hybrid screened exchange PBE_SOL version
    HYB_GGA_XC_HJS_PBE_SOL = 430,
    /// HJS hybrid screened exchange B88 version
    HYB_GGA_XC_HJS_B88 = 431,
    /// HJS hybrid screened exchange B97x version
    HYB_GGA_XC_HJS_B97X = 432,
    /// CAM version of B3LYP
    HYB_GGA_XC_CAM_B3LYP = 433,
    /// CAM version of B3LYP, tuned for excitations and properties
    HYB_GGA_XC_TUNED_CAM_B3LYP = 434,
    /// BHandH: 50% LDA exchange and 50% HF exchange with 100% LYP correlation
    HYB_GGA_XC_BHANDH = 435,
    /// BHandHLYP a.k.a. BHLYP: 50% B88 exchange and 50% HF exchange with 100%
    /// LYP correlation
    HYB_GGA_XC_BHANDHLYP = 436,
    /// B3LYP with RC04 LDA
    HYB_GGA_XC_MB3LYP_RC04 = 437,
    /// Minnesota M05 hybrid exchange functional
    HYB_MGGA_X_M05 = 438,
    /// Minnesota M05-2X hybrid exchange functional
    HYB_MGGA_X_M05_2X = 439,
    /// Mixture of B88 with BC95 (B1B95)
    HYB_MGGA_XC_B88B95 = 440,
    /// Mixture of B86 with BC95
    HYB_MGGA_XC_B86B95 = 441,
    /// Mixture of PW86 with BC95
    HYB_MGGA_XC_PW86B95 = 442,
    /// Mixture of B88 with BC95 from Zhao and Truhlar
    HYB_MGGA_XC_BB1K = 443,
    /// Minnesota M06-HF hybrid exchange functional
    HYB_MGGA_X_M06_HF = 444,
    /// Mixture of mPW91 with BC95 from Zhao and Truhlar
    HYB_MGGA_XC_MPW1B95 = 445,
    /// Mixture of mPW91 with BC95 for kinetics
    HYB_MGGA_XC_MPWB1K = 446,
    /// Mixture of X with BC95
    HYB_MGGA_XC_X1B95 = 447,
    /// Mixture of X with BC95 for kinetics
    HYB_MGGA_XC_XB1K = 448,
    /// Minnesota M06 hybrid exchange functional
    HYB_MGGA_X_M06 = 449,
    /// Minnesota M06-2X hybrid exchange functional
    HYB_MGGA_X_M06_2X = 450,
    /// Mixture of PW91 with BC95 from Zhao and Truhlar
    HYB_MGGA_XC_PW6B95 = 451,
    /// Mixture of PW91 with BC95 from Zhao and Truhlar for kinetics
    HYB_MGGA_XC_PWB6K = 452,
    /// MPW with 1 par. for metals/LYP
    HYB_GGA_XC_MPWLYP1M = 453,
    /// Revised B3LYP
    HYB_GGA_XC_REVB3LYP = 454,
    /// CAMY version of BLYP
    HYB_GGA_XC_CAMY_BLYP = 455,
    /// PBE0-1/3
    HYB_GGA_XC_PBE0_13 = 456,
    /// TPSSh
    HYB_MGGA_XC_TPSSH = 457,
    /// revTPSSh
    HYB_MGGA_XC_REVTPSSH = 458,
    /// B3LYP*
    HYB_GGA_XC_B3LYPS = 459,
    /// Global hybrid for vertical ionization potentials
    HYB_GGA_XC_QTP17 = 460,
    /// B3LYP-MCM1
    HYB_GGA_XC_B3LYP_MCM1 = 461,
    /// B3LYP-MCM2
    HYB_GGA_XC_B3LYP_MCM2 = 462,
    /// wB97 range-separated functional
    HYB_GGA_XC_WB97 = 463,
    /// wB97X range-separated functional
    HYB_GGA_XC_WB97X = 464,
    /// Long-range corrected short-range hybrid PBE (LRC-wPBEh) by Rohrdanz,
    /// Martins and Herbert
    HYB_GGA_XC_LRC_WPBEH = 465,
    /// wB97X-V range-separated functional
    HYB_GGA_XC_WB97X_V = 466,
    /// LCY version of PBE
    HYB_GGA_XC_LCY_PBE = 467,
    /// LCY version of BLYP
    HYB_GGA_XC_LCY_BLYP = 468,
    /// Vydrov and Van Voorhis
    HYB_GGA_XC_LC_VV10 = 469,
    /// CAMY version of B3LYP
    HYB_GGA_XC_CAMY_B3LYP = 470,
    /// wB97X-D range-separated functional
    HYB_GGA_XC_WB97X_D = 471,
    /// hPBEint
    HYB_GGA_XC_HPBEINT = 472,
    /// Long-range corrected PBE (LRC-wPBE) by Rohrdanz, Martins and Herbert
    HYB_GGA_XC_LRC_WPBE = 473,
    /// MVSh hybrid exchange functional
    HYB_MGGA_X_MVSH = 474,
    /// B3LYP with VWN functional 5 instead of RPA
    HYB_GGA_XC_B3LYP5 = 475,
    /// EDF2
    HYB_GGA_XC_EDF2 = 476,
    /// Correct Asymptotic Potential hybrid
    HYB_GGA_XC_CAP0 = 477,
    /// Long-range corrected PBE (LC-wPBE) by Vydrov and Scuseria
    HYB_GGA_XC_LC_WPBE = 478,
    /// HSE12
    HYB_GGA_XC_HSE12 = 479,
    /// HSE12 (short-range version)
    HYB_GGA_XC_HSE12S = 480,
    /// HSEsol
    HYB_GGA_XC_HSE_SOL = 481,
    /// CAM-B3LYP retuned using ionization potentials of water
    HYB_GGA_XC_CAM_QTP_01 = 482,
    /// mPW1LYP
    HYB_GGA_XC_MPW1LYP = 483,
    /// mPW1PBE
    HYB_GGA_XC_MPW1PBE = 484,
    /// Kang-Musgrave hybrid
    HYB_GGA_XC_KMLYP = 485,
    /// Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria
    HYB_GGA_XC_LC_WPBE_WHS = 486,
    /// Long-range corrected short-range hybrid PBE (LC-wPBE) by Weintraub,
    /// Henderson and Scuseria
    HYB_GGA_XC_LC_WPBEH_WHS = 487,
    /// Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria
    HYB_GGA_XC_LC_WPBE08_WHS = 488,
    /// Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria
    HYB_GGA_XC_LC_WPBESOL_WHS = 489,
    /// CAM-B3LYP retuned using ionization potentials of water
    HYB_GGA_XC_CAM_QTP_00 = 490,
    /// CAM-B3LYP retuned using ionization potentials of water
    HYB_GGA_XC_CAM_QTP_02 = 491,
    /// CAM-B3LYP retuned using ionization potentials of water
    HYB_GGA_XC_LC_QTP = 492,
    /// Regularized SCAN exchange by Bartok and Yates
    MGGA_X_RSCAN = 493,
    /// Regularized SCAN correlation by Bartok and Yates
    MGGA_C_RSCAN = 494,
    /// Swart 2012 GGA exchange
    GGA_X_S12G = 495,
    /// Swart 2012 hybrid GGA exchange
    HYB_GGA_X_S12H = 496,
    /// Re-regularized SCAN exchange by Furness et al
    MGGA_X_R2SCAN = 497,
    /// Re-regularized SCAN correlation by Furness et al
    MGGA_C_R2SCAN = 498,
    /// BLYP35
    HYB_GGA_XC_BLYP35 = 499,
    /// von Weiszaecker correction to Thomas-Fermi
    GGA_K_VW = 500,
    /// Second-order gradient expansion of the kinetic energy density
    GGA_K_GE2 = 501,
    /// TF-lambda-vW form by Golden (l = 13/45)
    GGA_K_GOLDEN = 502,
    /// TF-lambda-vW form by Yonei and Tomishima (l = 1/5)
    GGA_K_YT65 = 503,
    /// TF-lambda-vW form by Baltin (l = 5/9)
    GGA_K_BALTIN = 504,
    /// TF-lambda-vW form by Lieb (l = 0.185909191)
    GGA_K_LIEB = 505,
    /// gamma-TFvW form by Acharya et al [$g = 1 - 1.412/N^{1/3}$]
    GGA_K_ABSP1 = 506,
    /// gamma-TFvW form by Acharya et al [$g = 1 - 1.332/N^{1/3}$]
    GGA_K_ABSP2 = 507,
    /// gamma-TFvW form by Gazquez and Robles
    GGA_K_GR = 508,
    /// gamma-TFvW form by Ludena
    GGA_K_LUDENA = 509,
    /// gamma-TFvW form by Ghosh and Parr
    GGA_K_GP85 = 510,
    /// Pearson 1992
    GGA_K_PEARSON = 511,
    /// Ou-Yang and Levy v.1
    GGA_K_OL1 = 512,
    /// Ou-Yang and Levy v.2
    GGA_K_OL2 = 513,
    /// Fuentealba & Reyes (B88 version)
    GGA_K_FR_B88 = 514,
    /// Fuentealba & Reyes (PW86 version)
    GGA_K_FR_PW86 = 515,
    /// DePristo and Kress
    GGA_K_DK = 516,
    /// Perdew
    GGA_K_PERDEW = 517,
    /// Vitos, Skriver, and Kollar
    GGA_K_VSK = 518,
    /// Vitos, Johansson, Kollar, and Skriver
    GGA_K_VJKS = 519,
    /// Ernzerhof
    GGA_K_ERNZERHOF = 520,
    /// Lembarki & Chermette
    GGA_K_LC94 = 521,
    /// Lee, Lee & Parr
    GGA_K_LLP = 522,
    /// Thakkar 1992
    GGA_K_THAKKAR = 523,
    /// short-range part of the PBE (default w=0 gives PBEh)
    GGA_X_WPBEH = 524,
    /// HJS screened exchange PBE version
    GGA_X_HJS_PBE = 525,
    /// HJS screened exchange PBE_SOL version
    GGA_X_HJS_PBE_SOL = 526,
    /// HJS screened exchange B88 version
    GGA_X_HJS_B88 = 527,
    /// HJS screened exchange B97x version
    GGA_X_HJS_B97X = 528,
    /// Short-range recipe for B88 functional - erf
    GGA_X_ITYH = 529,
    /// Short-range recipe for B88 functional - Yukawa
    GGA_X_SFAT = 530,
    /// wB97M-V exchange-correlation functional
    HYB_MGGA_XC_WB97M_V = 531,
    /// Slater exchange with relativistic corrections
    LDA_X_REL = 532,
    /// Semiclassical GGA at fourth order
    GGA_X_SG4 = 533,
    /// Semiclassical GGA at fourth order
    GGA_C_SG4 = 534,
    /// Gilbert and Gill 1999
    GGA_X_GG99 = 535,
    /// LDA constructed from slab-like systems of 1 electron
    LDA_XC_1D_EHWLRG_1 = 536,
    /// LDA constructed from slab-like systems of 2 electrons
    LDA_XC_1D_EHWLRG_2 = 537,
    /// LDA constructed from slab-like systems of 3 electrons
    LDA_XC_1D_EHWLRG_3 = 538,
    /// PBE power
    GGA_X_PBEPOW = 539,
    /// Tao and Mo 2016 exchange
    MGGA_X_TM = 540,
    /// meta-GGA version of VT{8,4} GGA
    MGGA_X_VT84 = 541,
    /// TPSS with correct surface asymptotics
    MGGA_X_SA_TPSS = 542,
    /// Perdew and Constantin 2007
    MGGA_K_PC07 = 543,
    /// Gilbert and Gill 1999 (mixed)
    GGA_X_KGG99 = 544,
    /// high local exchange 2016
    GGA_XC_HLE16 = 545,
    /// Short-range LDA exchange with error function kernel (erfc)
    LDA_X_ERF = 546,
    /// Lee-Parr reparametrization A
    LDA_XC_LP_A = 547,
    /// Lee-Parr reparametrization B
    LDA_XC_LP_B = 548,
    /// Rae self-energy corrected exchange
    LDA_X_RAE = 549,
    /// Wigner including kinetic energy contribution
    LDA_K_ZLP = 550,
    /// McWeeny 76
    LDA_C_MCWEENY = 551,
    /// Brual & Rothstein 78
    LDA_C_BR78 = 552,
    /// GGA component of SCAN
    GGA_C_SCAN_E0 = 553,
    /// Proynov and Kong 2009
    LDA_C_PK09 = 554,
    /// GapC
    GGA_C_GAPC = 555,
    /// Gaploc
    GGA_C_GAPLOC = 556,
    /// another spin-dependent correction to PBEint
    GGA_C_ZVPBEINT = 557,
    /// another spin-dependent correction to PBEsol
    GGA_C_ZVPBESOL = 558,
    /// Thakkar and McCarthy reparametrization, also known as reLYP
    GGA_C_TM_LYP = 559,
    /// Thakkar and McCarthy reparametrization
    GGA_C_TM_PBE = 560,
    /// Wilson 94 (Eq. 25)
    GGA_C_W94 = 561,
    /// Krieger, Chen, Iafrate, and Savin
    MGGA_C_KCIS = 562,
    /// Hybrid based on KCIS
    HYB_MGGA_XC_B0KCIS = 563,
    /// Lee & Parr, Eq. (60)
    MGGA_XC_LP90 = 564,
    /// A dynamical correlation functional
    GGA_C_CS1 = 565,
    /// MPW1KCIS for barrier heights
    HYB_MGGA_XC_MPW1KCIS = 566,
    /// MPWKCIS1K for barrier heights
    HYB_MGGA_XC_MPWKCIS1K = 567,
    /// PBE1KCIS for binding energies
    HYB_MGGA_XC_PBE1KCIS = 568,
    /// TPSS1KCIS for thermochemistry and kinetics
    HYB_MGGA_XC_TPSS1KCIS = 569,
    /// Becke 88 reoptimized to be used with tau1
    GGA_X_B88M = 570,
    /// Meta-GGA correlation by Becke
    MGGA_C_B88 = 571,
    /// B5050LYP
    HYB_GGA_XC_B5050LYP = 572,
    /// Wigner with corresponding LYP parameters
    LDA_C_OW_LYP = 573,
    /// Optimized Wigner
    LDA_C_OW = 574,
    /// GX functional of Loos
    MGGA_X_GX = 575,
    /// PBE-GX functional of Loos
    MGGA_X_PBE_GX = 576,
    /// Groth, Dornheim, Sjostrom, Malone, Foulkes, Bonitz
    LDA_XC_GDSMFB = 577,
    /// Gordon and Kim 1972
    LDA_C_GK72 = 578,
    /// Karasiev reparameterization of Chachiyo
    LDA_C_KARASIEV = 579,
    /// Liu-Parr kinetic
    LDA_K_LP96 = 580,
    /// revised SCAN
    MGGA_X_REVSCAN = 581,
    /// revised SCAN
    MGGA_C_REVSCAN = 582,
    /// revised SCAN hybrid exchange (SCAN0)
    HYB_MGGA_X_REVSCAN0 = 583,
    /// SCAN + VV10 correlation
    MGGA_C_SCAN_VV10 = 584,
    /// REVSCAN + VV10 correlation
    MGGA_C_REVSCAN_VV10 = 585,
    /// Becke-Roussel 89 with an explicit inversion of x(y), gamma = 0.8
    MGGA_X_BR89_EXPLICIT = 586,
    /// Keal and Tozer, version 3
    GGA_XC_KT3 = 587,
    /// Baer and Neuhauser, gamma=1
    HYB_LDA_XC_BN05 = 588,
    /// Livshits and Baer, empirical functional also used for IP tuning
    HYB_GGA_XC_LB07 = 589,
    /// Long-range LDA correlation functional
    LDA_C_PMGB06 = 590,
    /// Combined analytical theory with Monte Carlo sampling
    GGA_K_GDS08 = 591,
    /// As GDS08 but for an electron gas with spin
    GGA_K_GHDS10 = 592,
    /// Reparametrized GHDS10
    GGA_K_GHDS10R = 593,
    /// Trickey, Karasiev, and Vela
    GGA_K_TKVLN = 594,
    /// Three parameter PBE-like expansion
    GGA_K_PBE3 = 595,
    /// Four parameter PBE-like expansion
    GGA_K_PBE4 = 596,
    /// Intermediate form between PBE3 and PBE4
    GGA_K_EXP4 = 597,
    /// Becke 98
    HYB_MGGA_XC_B98 = 598,
    /// Neural network LDA from Tozer et al
    LDA_XC_TIH = 599,
    /// Exchange in 1D for an exponentially screened interaction
    LDA_X_1D_EXPONENTIAL = 600,
    /// Short-range recipe for PBE functional - Yukawa
    GGA_X_SFAT_PBE = 601,
    /// Becke-Roussel 89 with an explicit inversion of x(y), gamma = 1.0
    MGGA_X_BR89_EXPLICIT_1 = 602,
    /// Regularized TPSS
    MGGA_X_REGTPSS = 603,
    /// Functional derivative recovered from the stray LB94 potential
    GGA_X_FD_LB94 = 604,
    /// Revised FD_LB94
    GGA_X_FD_REVLB94 = 605,
    /// PBEloc variation with enhanced compatibility with exact exchange
    GGA_C_ZVPBELOC = 606,
    /// Hybrid based on APBE
    HYB_GGA_XC_APBE0 = 607,
    /// Hybrid based in APBE and zvPBEloc
    HYB_GGA_XC_HAPBE = 608,
    /// JS17 meta-GGA for 2D
    MGGA_X_2D_JS17 = 609,
    /// Similar to CAM-B3LYP, but trying to reduce the many-electron
    /// self-interaction
    HYB_GGA_XC_RCAM_B3LYP = 610,
    /// hybrid fitted to carbon NMR shifts
    HYB_GGA_XC_WC04 = 611,
    /// hybrid fitted to proton NMR shifts
    HYB_GGA_XC_WP04 = 612,
    /// Luo-Karasiev-Trickey GGA kinetic
    GGA_K_LKT = 613,
    /// CAM version of B3LYP, tuned for TDDFT
    HYB_GGA_XC_CAMH_B3LYP = 614,
    /// Long-range corrected short-range hybrid PBE (whPBE0) by Shao et al
    HYB_GGA_XC_WHPBE0 = 615,
    /// Three parameter PBE-like expansion
    GGA_K_PBE2 = 616,
    /// L0.4 by Laricchia et al
    MGGA_K_L04 = 617,
    /// L0.6 by Laricchia et al
    MGGA_K_L06 = 618,
    /// VT84F by Karasiev et al
    GGA_K_VT84F = 619,
    /// LGAP by Constantin et al
    GGA_K_LGAP = 620,
    /// Reduced derivative approximation by Karasiev et al
    MGGA_K_RDA = 621,
    /// Short-range recipe for OPTX functional
    GGA_X_ITYH_OPTX = 622,
    /// Short-range recipe for PBE functional
    GGA_X_ITYH_PBE = 623,
    /// Short-range LYP by Ai, Fang, and Su
    GGA_C_LYPR = 624,
    /// LC version of BLYP for electron affinities
    HYB_GGA_XC_LC_BLYP_EA = 625,
    /// Regularized Tao and Mo exchange
    MGGA_X_REGTM = 626,
    /// Second-order gradient expansion
    MGGA_K_GEA2 = 627,
    /// Fourth-order gradient expansion
    MGGA_K_GEA4 = 628,
    /// mGGA-rev functional by Cancio, Stewart, and Kuna (a=1)
    MGGA_K_CSK1 = 629,
    /// mGGA-rev functional by Cancio, Stewart, and Kuna (a=4)
    MGGA_K_CSK4 = 630,
    /// mGGAloc-rev functional by Cancio, Stewart, and Kuna (a=1)
    MGGA_K_CSK_LOC1 = 631,
    /// mGGAloc-rev functional by Cancio, Stewart, and Kuna (a=4)
    MGGA_K_CSK_LOC4 = 632,
    /// LGAP-GE by Constantin et al
    GGA_K_LGAP_GE = 633,
    /// Reoptimized PC07 by Mejia-Rodriguez and Trickey
    MGGA_K_PC07_OPT = 634,
    /// empirically optimized gamma-TFvW form
    GGA_K_TFVW_OPT = 635,
    /// LC version of B88
    HYB_GGA_XC_LC_BOP = 636,
    /// LC version of PBE
    HYB_GGA_XC_LC_PBEOP = 637,
    /// Krieger, Chen, and Kurth
    MGGA_C_KCISK = 638,
    /// LC version of BLYP with correlation only in the short range
    HYB_GGA_XC_LC_BLYPR = 639,
    /// Modified CAM-B3LYP by Day, Nguyen and Pachter
    HYB_GGA_XC_MCAM_B3LYP = 640,
    /// Short-range LDA exchange with Yukawa attenuation
    LDA_X_YUKAWA = 641,
    /// Re-regularized SCAN correlation with larger value for eta
    MGGA_C_R2SCAN01 = 642,
    /// Revised correlation energy for MGGAC exchange functional
    MGGA_C_RMGGAC = 643,
    /// MCML exchange
    MGGA_X_MCML = 644,
    /// Re-regularized SCAN exchange by Furness et al with larger value for eta
    MGGA_X_R2SCAN01 = 645,
    /// Swart 2012 range-separated hybrid GGA exchange
    HYB_GGA_X_CAM_S12G = 646,
    /// Swart 2012 range-separated hybrid GGA exchange
    HYB_GGA_X_CAM_S12H = 647,
    /// r++SCAN: rSCAN with uniform density limit and coordinate scaling
    /// behavior
    MGGA_X_RPPSCAN = 648,
    /// r++SCAN: rSCAN with uniform density limit and coordinate scaling
    /// behavior
    MGGA_C_RPPSCAN = 649,
    /// r$^{4}$SCAN, a functional that satisfies the same exact constraints that
    /// SCAN does
    MGGA_X_R4SCAN = 650,
    /// Exchange part of VCML-rVV10 by Trepte and Voss
    MGGA_X_VCML = 651,
    /// VCML-rVV10 by Trepte and Voss
    MGGA_XC_VCML_RVV10 = 652,
    /// Long-range corrected functional based on short-range LDA exchange (erfc)
    HYB_LDA_X_ERF = 653,
    /// Short ranged correlation LDA (erfc)
    LDA_C_PW_ERF = 654,
    /// Short ranged PBE exchange (erfc)
    GGA_X_PBE_ERF_GWS = 655,
    /// Short-range PBE (GWS) exchange (erfc) + long-range exact exchange
    HYB_GGA_X_PBE_ERF_GWS = 656,
    /// Short ranged PBE correlation (erfc)
    GGA_C_PBE_ERF_GWS = 657,
    /// Google Accelerated Science 22
    HYB_MGGA_XC_GAS22 = 658,
    /// r2SCANh: r2SCAN hybrid like TPSSh with 10% exact exchange
    HYB_MGGA_XC_R2SCANH = 659,
    /// r2SCAN0: r2SCAN hybrid like PBE0 with 25% exact exchange
    HYB_MGGA_XC_R2SCAN0 = 660,
    /// r2SCAN50: r2SCAN hybrid like PBE50 with 50% exact exchange
    HYB_MGGA_XC_R2SCAN50 = 661,
    /// Range-separated re-regularized SCAN exchange by Wittmann et al
    HYB_MGGA_X_WR2SCAN = 662,
    /// CAM hybrid screened exchange PBE version
    HYB_GGA_XC_CAM_PBEH = 681,
    /// CAMY hybrid screened exchange PBE version
    HYB_GGA_XC_CAMY_PBEH = 682,
    /// Ruggeri, Rios, and Alavi unrestricted fit
    LDA_C_UPW92 = 683,
    /// Ruggeri, Rios, and Alavi restricted fit
    LDA_C_RPW92 = 684,
    /// LDA-type exchange with tau-dependent potential
    MGGA_X_TLDA = 685,
    /// Tao 2001
    MGGA_X_EDMGGA = 686,
    /// Generalized density-matrix with a=1/2
    MGGA_X_GDME_NV = 687,
    /// Reparametrized local-density approximation
    MGGA_X_RLDA = 688,
    /// Generalized density-matrix with a=0
    MGGA_X_GDME_0 = 689,
    /// Generalized density-matrix with a=0.00638
    MGGA_X_GDME_KOS = 690,
    /// Varied-terms (VT) mGGA of Koehl, Odom, and Scuseria
    MGGA_X_GDME_VT = 691,
    /// simple local model for Slater potential
    LDA_X_SLOC = 692,
    /// revised Tao and Mo 2016 exchange
    MGGA_X_REVTM = 693,
    /// revised Tao and Mo 2016 exchange
    MGGA_C_REVTM = 694,
    /// EDMGGA hybrid
    HYB_MGGA_XC_EDMGGAH = 695,
    /// Modified Becke-Roussel for band gaps - cuspless hole
    MGGA_X_MBRXC_BG = 696,
    /// Modified Becke-Roussel for band gaps - hydrogen hole
    MGGA_X_MBRXH_BG = 697,
    /// Half-and-half meta-LDAized LDA exchange by Lehtola and Marques
    MGGA_X_HLTA = 698,
    /// Half-and-half meta-LDAized PW correlation by Lehtola and Marques
    MGGA_C_HLTAPW = 699,
    /// Deorbitalized SCAN (SCAN-L) exchange
    MGGA_X_SCANL = 700,
    /// Deorbitalized revised SCAN (revSCAN-L) exchange
    MGGA_X_REVSCANL = 701,
    /// Deorbitalized SCAN (SCAN-L) correlation
    MGGA_C_SCANL = 702,
    /// SCAN-L + rVV10 correlation
    MGGA_C_SCANL_RVV10 = 703,
    /// SCAN-L + VV10 correlation
    MGGA_C_SCANL_VV10 = 704,
    /// Jana and Samal 2018, screened range-separated TM exchange
    HYB_MGGA_X_JS18 = 705,
    /// Patra, Jana and Samal 2018, screened range-separated TM exchange
    HYB_MGGA_X_PJS18 = 706,
    /// TASK exchange of Aschebrock and Kuemmel
    MGGA_X_TASK = 707,
    /// MGGAC exchange of Patra et al
    MGGA_X_MGGAC = 711,
    /// beta fitted to LC20 to be used with MGGAC
    GGA_C_MGGAC = 712,
    /// modified Becke-Roussel by Patra et al
    MGGA_X_MBR = 716,
    /// Deorbitalized re-regularized SCAN (r2SCAN-L) exchange
    MGGA_X_R2SCANL = 718,
    /// Deorbitalized re-regularized SCAN (r2SCAN-L) correlation
    MGGA_C_R2SCANL = 719,
    /// Long-range corrected TM-LYP by Jana et al
    HYB_MGGA_XC_LC_TMLYP = 720,
    /// modified TASK exchange
    MGGA_X_MTASK = 724,
    /// Functional for quasi-1D systems
    GGA_X_Q1D = 734,
    /// KTBM learned exchange - 0
    MGGA_X_KTBM_0 = 735,
    /// KTBM learned exchange - 1
    MGGA_X_KTBM_1 = 736,
    /// KTBM learned exchange - 2
    MGGA_X_KTBM_2 = 737,
    /// KTBM learned exchange - 3
    MGGA_X_KTBM_3 = 738,
    /// KTBM learned exchange - 4
    MGGA_X_KTBM_4 = 739,
    /// KTBM learned exchange - 5
    MGGA_X_KTBM_5 = 740,
    /// KTBM learned exchange - 6
    MGGA_X_KTBM_6 = 741,
    /// KTBM learned exchange - 7
    MGGA_X_KTBM_7 = 742,
    /// KTBM learned exchange - 8
    MGGA_X_KTBM_8 = 743,
    /// KTBM learned exchange - 9
    MGGA_X_KTBM_9 = 744,
    /// KTBM learned exchange - 10
    MGGA_X_KTBM_10 = 745,
    /// KTBM learned exchange - 11
    MGGA_X_KTBM_11 = 746,
    /// KTBM learned exchange - 12
    MGGA_X_KTBM_12 = 747,
    /// KTBM learned exchange - 13
    MGGA_X_KTBM_13 = 748,
    /// KTBM learned exchange - 14
    MGGA_X_KTBM_14 = 749,
    /// KTBM learned exchange - 15
    MGGA_X_KTBM_15 = 750,
    /// KTBM learned exchange - 16
    MGGA_X_KTBM_16 = 751,
    /// KTBM learned exchange - 17
    MGGA_X_KTBM_17 = 752,
    /// KTBM learned exchange - 18
    MGGA_X_KTBM_18 = 753,
    /// KTBM learned exchange - 19
    MGGA_X_KTBM_19 = 754,
    /// KTBM learned exchange - 20
    MGGA_X_KTBM_20 = 755,
    /// KTBM learned exchange - 21
    MGGA_X_KTBM_21 = 756,
    /// KTBM learned exchange - 22
    MGGA_X_KTBM_22 = 757,
    /// KTBM learned exchange - 23
    MGGA_X_KTBM_23 = 758,
    /// KTBM learned exchange - 24
    MGGA_X_KTBM_24 = 759,
    /// KTBM learned exchange - GAP
    MGGA_X_KTBM_GAP = 760,
    /// MS-PBEl, a PBE-like meta-GGA exchange
    MGGA_X_MSPBEL = 761,
    /// regularized MS-PBEl
    MGGA_X_RMSPBEL = 762,
    /// MS-RPBEl, a RPBE-like meta-GGA exchange
    MGGA_X_MSRPBEL = 763,
    /// regularized MS-RPBEl
    MGGA_X_RMSRPBEL = 764,
    /// MS-B86bl, a B86b-like meta-GGA exchange
    MGGA_X_MSB86BL = 765,
    /// regularized MS-B86bl
    MGGA_X_RMSB86BL = 766,
    /// Dispersionless physically-informed Minnesota M06-2X hybrid exchange
    /// functional
    HYB_MGGA_X_PI_M06_2X_DL = 767,
    /// Dispersionless physically-informed Minnesota M06-2X correlation
    /// functional
    MGGA_C_PI_M06_2X_DL = 768,
    /// Physically-informed Minnesota M06-2X hybrid exchange functional
    HYB_MGGA_X_PI_M06_2X = 769,
    /// Physically-informed Minnesota M06-2X correlation functional
    MGGA_C_PI_M06_2X = 770,
}

impl XcFuncId {
    /// Get the numeric value of this functional ID.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Get the C-style name string (e.g., "LDA_X", "GGA_X_PBE").
    pub fn name(self) -> &'static str {
        match self {
            XcFuncId::LDA_X => "LDA_X",
            XcFuncId::LDA_C_WIGNER => "LDA_C_WIGNER",
            XcFuncId::LDA_C_RPA => "LDA_C_RPA",
            XcFuncId::LDA_C_HL => "LDA_C_HL",
            XcFuncId::LDA_C_GL => "LDA_C_GL",
            XcFuncId::LDA_C_XALPHA => "LDA_C_XALPHA",
            XcFuncId::LDA_C_VWN => "LDA_C_VWN",
            XcFuncId::LDA_C_VWN_RPA => "LDA_C_VWN_RPA",
            XcFuncId::LDA_C_PZ => "LDA_C_PZ",
            XcFuncId::LDA_C_PZ_MOD => "LDA_C_PZ_MOD",
            XcFuncId::LDA_C_OB_PZ => "LDA_C_OB_PZ",
            XcFuncId::LDA_C_PW => "LDA_C_PW",
            XcFuncId::LDA_C_PW_MOD => "LDA_C_PW_MOD",
            XcFuncId::LDA_C_OB_PW => "LDA_C_OB_PW",
            XcFuncId::LDA_C_2D_AMGB => "LDA_C_2D_AMGB",
            XcFuncId::LDA_C_2D_PRM => "LDA_C_2D_PRM",
            XcFuncId::LDA_C_VBH => "LDA_C_VBH",
            XcFuncId::LDA_C_1D_CSS => "LDA_C_1D_CSS",
            XcFuncId::LDA_X_2D => "LDA_X_2D",
            XcFuncId::LDA_XC_TETER93 => "LDA_XC_TETER93",
            XcFuncId::LDA_X_1D_SOFT => "LDA_X_1D_SOFT",
            XcFuncId::LDA_C_ML1 => "LDA_C_ML1",
            XcFuncId::LDA_C_ML2 => "LDA_C_ML2",
            XcFuncId::LDA_C_GOMBAS => "LDA_C_GOMBAS",
            XcFuncId::LDA_C_PW_RPA => "LDA_C_PW_RPA",
            XcFuncId::LDA_C_1D_LOOS => "LDA_C_1D_LOOS",
            XcFuncId::LDA_C_RC04 => "LDA_C_RC04",
            XcFuncId::LDA_C_VWN_1 => "LDA_C_VWN_1",
            XcFuncId::LDA_C_VWN_2 => "LDA_C_VWN_2",
            XcFuncId::LDA_C_VWN_3 => "LDA_C_VWN_3",
            XcFuncId::LDA_C_VWN_4 => "LDA_C_VWN_4",
            XcFuncId::GGA_X_GAM => "GGA_X_GAM",
            XcFuncId::GGA_C_GAM => "GGA_C_GAM",
            XcFuncId::GGA_X_HCTH_A => "GGA_X_HCTH_A",
            XcFuncId::GGA_X_EV93 => "GGA_X_EV93",
            XcFuncId::HYB_MGGA_X_DLDF => "HYB_MGGA_X_DLDF",
            XcFuncId::MGGA_C_DLDF => "MGGA_C_DLDF",
            XcFuncId::GGA_X_BCGP => "GGA_X_BCGP",
            XcFuncId::GGA_C_ACGGA => "GGA_C_ACGGA",
            XcFuncId::GGA_X_LAMBDA_OC2_N => "GGA_X_LAMBDA_OC2_N",
            XcFuncId::GGA_X_B86_R => "GGA_X_B86_R",
            XcFuncId::MGGA_XC_ZLP => "MGGA_XC_ZLP",
            XcFuncId::LDA_XC_ZLP => "LDA_XC_ZLP",
            XcFuncId::GGA_X_LAMBDA_CH_N => "GGA_X_LAMBDA_CH_N",
            XcFuncId::GGA_X_LAMBDA_LO_N => "GGA_X_LAMBDA_LO_N",
            XcFuncId::GGA_X_HJS_B88_V2 => "GGA_X_HJS_B88_V2",
            XcFuncId::GGA_C_Q2D => "GGA_C_Q2D",
            XcFuncId::GGA_X_Q2D => "GGA_X_Q2D",
            XcFuncId::GGA_X_PBE_MOL => "GGA_X_PBE_MOL",
            XcFuncId::LDA_K_TF => "LDA_K_TF",
            XcFuncId::LDA_K_LP => "LDA_K_LP",
            XcFuncId::GGA_K_TFVW => "GGA_K_TFVW",
            XcFuncId::GGA_K_REVAPBEINT => "GGA_K_REVAPBEINT",
            XcFuncId::GGA_K_APBEINT => "GGA_K_APBEINT",
            XcFuncId::GGA_K_REVAPBE => "GGA_K_REVAPBE",
            XcFuncId::GGA_X_AK13 => "GGA_X_AK13",
            XcFuncId::GGA_K_MEYER => "GGA_K_MEYER",
            XcFuncId::GGA_X_LV_RPW86 => "GGA_X_LV_RPW86",
            XcFuncId::GGA_X_PBE_TCA => "GGA_X_PBE_TCA",
            XcFuncId::GGA_X_PBEINT => "GGA_X_PBEINT",
            XcFuncId::GGA_C_ZPBEINT => "GGA_C_ZPBEINT",
            XcFuncId::GGA_C_PBEINT => "GGA_C_PBEINT",
            XcFuncId::GGA_C_ZPBESOL => "GGA_C_ZPBESOL",
            XcFuncId::MGGA_XC_OTPSS_D => "MGGA_XC_OTPSS_D",
            XcFuncId::GGA_XC_OPBE_D => "GGA_XC_OPBE_D",
            XcFuncId::GGA_XC_OPWLYP_D => "GGA_XC_OPWLYP_D",
            XcFuncId::GGA_XC_OBLYP_D => "GGA_XC_OBLYP_D",
            XcFuncId::GGA_X_VMT84_GE => "GGA_X_VMT84_GE",
            XcFuncId::GGA_X_VMT84_PBE => "GGA_X_VMT84_PBE",
            XcFuncId::GGA_X_VMT_GE => "GGA_X_VMT_GE",
            XcFuncId::GGA_X_VMT_PBE => "GGA_X_VMT_PBE",
            XcFuncId::MGGA_C_CS => "MGGA_C_CS",
            XcFuncId::MGGA_C_MN12_SX => "MGGA_C_MN12_SX",
            XcFuncId::MGGA_C_MN12_L => "MGGA_C_MN12_L",
            XcFuncId::MGGA_C_M11_L => "MGGA_C_M11_L",
            XcFuncId::MGGA_C_M11 => "MGGA_C_M11",
            XcFuncId::MGGA_C_M08_SO => "MGGA_C_M08_SO",
            XcFuncId::MGGA_C_M08_HX => "MGGA_C_M08_HX",
            XcFuncId::GGA_C_N12_SX => "GGA_C_N12_SX",
            XcFuncId::GGA_C_N12 => "GGA_C_N12",
            XcFuncId::HYB_GGA_X_N12_SX => "HYB_GGA_X_N12_SX",
            XcFuncId::GGA_X_N12 => "GGA_X_N12",
            XcFuncId::GGA_C_REGTPSS => "GGA_C_REGTPSS",
            XcFuncId::GGA_C_OP_XALPHA => "GGA_C_OP_XALPHA",
            XcFuncId::GGA_C_OP_G96 => "GGA_C_OP_G96",
            XcFuncId::GGA_C_OP_PBE => "GGA_C_OP_PBE",
            XcFuncId::GGA_C_OP_B88 => "GGA_C_OP_B88",
            XcFuncId::GGA_C_FT97 => "GGA_C_FT97",
            XcFuncId::GGA_C_SPBE => "GGA_C_SPBE",
            XcFuncId::GGA_X_SSB_SW => "GGA_X_SSB_SW",
            XcFuncId::GGA_X_SSB => "GGA_X_SSB",
            XcFuncId::GGA_X_SSB_D => "GGA_X_SSB_D",
            XcFuncId::GGA_XC_HCTH_407P => "GGA_XC_HCTH_407P",
            XcFuncId::GGA_XC_HCTH_P76 => "GGA_XC_HCTH_P76",
            XcFuncId::GGA_XC_HCTH_P14 => "GGA_XC_HCTH_P14",
            XcFuncId::GGA_XC_B97_GGA1 => "GGA_XC_B97_GGA1",
            XcFuncId::GGA_C_HCTH_A => "GGA_C_HCTH_A",
            XcFuncId::GGA_X_BPCCAC => "GGA_X_BPCCAC",
            XcFuncId::GGA_C_REVTCA => "GGA_C_REVTCA",
            XcFuncId::GGA_C_TCA => "GGA_C_TCA",
            XcFuncId::GGA_X_PBE => "GGA_X_PBE",
            XcFuncId::GGA_X_PBE_R => "GGA_X_PBE_R",
            XcFuncId::GGA_X_B86 => "GGA_X_B86",
            XcFuncId::HYB_LDA_XC_B93 => "HYB_LDA_XC_B93",
            XcFuncId::GGA_X_B86_MGC => "GGA_X_B86_MGC",
            XcFuncId::GGA_X_B88 => "GGA_X_B88",
            XcFuncId::GGA_X_G96 => "GGA_X_G96",
            XcFuncId::GGA_X_PW86 => "GGA_X_PW86",
            XcFuncId::GGA_X_PW91 => "GGA_X_PW91",
            XcFuncId::GGA_X_OPTX => "GGA_X_OPTX",
            XcFuncId::GGA_X_DK87_R1 => "GGA_X_DK87_R1",
            XcFuncId::GGA_X_DK87_R2 => "GGA_X_DK87_R2",
            XcFuncId::GGA_X_LG93 => "GGA_X_LG93",
            XcFuncId::GGA_X_FT97_A => "GGA_X_FT97_A",
            XcFuncId::GGA_X_FT97_B => "GGA_X_FT97_B",
            XcFuncId::GGA_X_PBE_SOL => "GGA_X_PBE_SOL",
            XcFuncId::GGA_X_RPBE => "GGA_X_RPBE",
            XcFuncId::GGA_X_WC => "GGA_X_WC",
            XcFuncId::GGA_X_MPW91 => "GGA_X_MPW91",
            XcFuncId::GGA_X_AM05 => "GGA_X_AM05",
            XcFuncId::GGA_X_PBEA => "GGA_X_PBEA",
            XcFuncId::GGA_X_MPBE => "GGA_X_MPBE",
            XcFuncId::GGA_X_XPBE => "GGA_X_XPBE",
            XcFuncId::GGA_X_2D_B86_MGC => "GGA_X_2D_B86_MGC",
            XcFuncId::GGA_X_BAYESIAN => "GGA_X_BAYESIAN",
            XcFuncId::GGA_X_PBE_JSJR => "GGA_X_PBE_JSJR",
            XcFuncId::GGA_X_2D_B88 => "GGA_X_2D_B88",
            XcFuncId::GGA_X_2D_B86 => "GGA_X_2D_B86",
            XcFuncId::GGA_X_2D_PBE => "GGA_X_2D_PBE",
            XcFuncId::GGA_C_PBE => "GGA_C_PBE",
            XcFuncId::GGA_C_LYP => "GGA_C_LYP",
            XcFuncId::GGA_C_P86 => "GGA_C_P86",
            XcFuncId::GGA_C_PBE_SOL => "GGA_C_PBE_SOL",
            XcFuncId::GGA_C_PW91 => "GGA_C_PW91",
            XcFuncId::GGA_C_AM05 => "GGA_C_AM05",
            XcFuncId::GGA_C_XPBE => "GGA_C_XPBE",
            XcFuncId::GGA_C_LM => "GGA_C_LM",
            XcFuncId::GGA_C_PBE_JRGX => "GGA_C_PBE_JRGX",
            XcFuncId::GGA_X_OPTB88_VDW => "GGA_X_OPTB88_VDW",
            XcFuncId::GGA_X_PBEK1_VDW => "GGA_X_PBEK1_VDW",
            XcFuncId::GGA_X_OPTPBE_VDW => "GGA_X_OPTPBE_VDW",
            XcFuncId::GGA_X_RGE2 => "GGA_X_RGE2",
            XcFuncId::GGA_C_RGE2 => "GGA_C_RGE2",
            XcFuncId::GGA_X_RPW86 => "GGA_X_RPW86",
            XcFuncId::GGA_X_KT1 => "GGA_X_KT1",
            XcFuncId::GGA_XC_KT2 => "GGA_XC_KT2",
            XcFuncId::GGA_C_WL => "GGA_C_WL",
            XcFuncId::GGA_C_WI => "GGA_C_WI",
            XcFuncId::GGA_X_MB88 => "GGA_X_MB88",
            XcFuncId::GGA_X_SOGGA => "GGA_X_SOGGA",
            XcFuncId::GGA_X_SOGGA11 => "GGA_X_SOGGA11",
            XcFuncId::GGA_C_SOGGA11 => "GGA_C_SOGGA11",
            XcFuncId::GGA_C_WI0 => "GGA_C_WI0",
            XcFuncId::GGA_XC_TH1 => "GGA_XC_TH1",
            XcFuncId::GGA_XC_TH2 => "GGA_XC_TH2",
            XcFuncId::GGA_XC_TH3 => "GGA_XC_TH3",
            XcFuncId::GGA_XC_TH4 => "GGA_XC_TH4",
            XcFuncId::GGA_X_C09X => "GGA_X_C09X",
            XcFuncId::GGA_C_SOGGA11_X => "GGA_C_SOGGA11_X",
            XcFuncId::GGA_X_LB => "GGA_X_LB",
            XcFuncId::GGA_XC_HCTH_93 => "GGA_XC_HCTH_93",
            XcFuncId::GGA_XC_HCTH_120 => "GGA_XC_HCTH_120",
            XcFuncId::GGA_XC_HCTH_147 => "GGA_XC_HCTH_147",
            XcFuncId::GGA_XC_HCTH_407 => "GGA_XC_HCTH_407",
            XcFuncId::GGA_XC_EDF1 => "GGA_XC_EDF1",
            XcFuncId::GGA_XC_XLYP => "GGA_XC_XLYP",
            XcFuncId::GGA_XC_KT1 => "GGA_XC_KT1",
            XcFuncId::GGA_X_LSPBE => "GGA_X_LSPBE",
            XcFuncId::GGA_X_LSRPBE => "GGA_X_LSRPBE",
            XcFuncId::GGA_XC_B97_D => "GGA_XC_B97_D",
            XcFuncId::GGA_X_OPTB86B_VDW => "GGA_X_OPTB86B_VDW",
            XcFuncId::MGGA_C_REVM11 => "MGGA_C_REVM11",
            XcFuncId::GGA_XC_PBE1W => "GGA_XC_PBE1W",
            XcFuncId::GGA_XC_MPWLYP1W => "GGA_XC_MPWLYP1W",
            XcFuncId::GGA_XC_PBELYP1W => "GGA_XC_PBELYP1W",
            XcFuncId::GGA_C_ACGGAP => "GGA_C_ACGGAP",
            XcFuncId::HYB_LDA_XC_LDA0 => "HYB_LDA_XC_LDA0",
            XcFuncId::HYB_LDA_XC_CAM_LDA0 => "HYB_LDA_XC_CAM_LDA0",
            XcFuncId::GGA_X_B88_6311G => "GGA_X_B88_6311G",
            XcFuncId::GGA_X_NCAP => "GGA_X_NCAP",
            XcFuncId::GGA_XC_NCAP => "GGA_XC_NCAP",
            XcFuncId::GGA_X_LBM => "GGA_X_LBM",
            XcFuncId::GGA_X_OL2 => "GGA_X_OL2",
            XcFuncId::GGA_X_APBE => "GGA_X_APBE",
            XcFuncId::GGA_K_APBE => "GGA_K_APBE",
            XcFuncId::GGA_C_APBE => "GGA_C_APBE",
            XcFuncId::GGA_K_TW1 => "GGA_K_TW1",
            XcFuncId::GGA_K_TW2 => "GGA_K_TW2",
            XcFuncId::GGA_K_TW3 => "GGA_K_TW3",
            XcFuncId::GGA_K_TW4 => "GGA_K_TW4",
            XcFuncId::GGA_X_HTBS => "GGA_X_HTBS",
            XcFuncId::GGA_X_AIRY => "GGA_X_AIRY",
            XcFuncId::GGA_X_LAG => "GGA_X_LAG",
            XcFuncId::GGA_XC_MOHLYP => "GGA_XC_MOHLYP",
            XcFuncId::GGA_XC_MOHLYP2 => "GGA_XC_MOHLYP2",
            XcFuncId::LDA_XC_TH_FL => "LDA_XC_TH_FL",
            XcFuncId::GGA_XC_TH_FC => "GGA_XC_TH_FC",
            XcFuncId::GGA_XC_TH_FCFO => "GGA_XC_TH_FCFO",
            XcFuncId::GGA_XC_TH_FCO => "GGA_XC_TH_FCO",
            XcFuncId::GGA_C_OPTC => "GGA_C_OPTC",
            XcFuncId::MGGA_X_LTA => "MGGA_X_LTA",
            XcFuncId::MGGA_X_TPSS => "MGGA_X_TPSS",
            XcFuncId::MGGA_X_M06_L => "MGGA_X_M06_L",
            XcFuncId::MGGA_X_GVT4 => "MGGA_X_GVT4",
            XcFuncId::MGGA_X_TAU_HCTH => "MGGA_X_TAU_HCTH",
            XcFuncId::MGGA_X_BR89 => "MGGA_X_BR89",
            XcFuncId::MGGA_X_BJ06 => "MGGA_X_BJ06",
            XcFuncId::MGGA_X_TB09 => "MGGA_X_TB09",
            XcFuncId::MGGA_X_RPP09 => "MGGA_X_RPP09",
            XcFuncId::MGGA_X_2D_PRHG07 => "MGGA_X_2D_PRHG07",
            XcFuncId::MGGA_X_2D_PRHG07_PRP10 => "MGGA_X_2D_PRHG07_PRP10",
            XcFuncId::MGGA_X_REVTPSS => "MGGA_X_REVTPSS",
            XcFuncId::MGGA_X_PKZB => "MGGA_X_PKZB",
            XcFuncId::MGGA_X_BR89_1 => "MGGA_X_BR89_1",
            XcFuncId::GGA_X_ECMV92 => "GGA_X_ECMV92",
            XcFuncId::GGA_C_PBE_VWN => "GGA_C_PBE_VWN",
            XcFuncId::GGA_C_P86_FT => "GGA_C_P86_FT",
            XcFuncId::GGA_K_RATIONAL_P => "GGA_K_RATIONAL_P",
            XcFuncId::GGA_K_PG1 => "GGA_K_PG1",
            XcFuncId::MGGA_K_PGSL025 => "MGGA_K_PGSL025",
            XcFuncId::MGGA_X_MS0 => "MGGA_X_MS0",
            XcFuncId::MGGA_X_MS1 => "MGGA_X_MS1",
            XcFuncId::MGGA_X_MS2 => "MGGA_X_MS2",
            XcFuncId::HYB_MGGA_X_MS2H => "HYB_MGGA_X_MS2H",
            XcFuncId::MGGA_X_TH => "MGGA_X_TH",
            XcFuncId::MGGA_X_M11_L => "MGGA_X_M11_L",
            XcFuncId::MGGA_X_MN12_L => "MGGA_X_MN12_L",
            XcFuncId::MGGA_X_MS2_REV => "MGGA_X_MS2_REV",
            XcFuncId::MGGA_XC_CC06 => "MGGA_XC_CC06",
            XcFuncId::MGGA_X_GP86 => "MGGA_X_GP86",
            XcFuncId::MGGA_C_TPSS => "MGGA_C_TPSS",
            XcFuncId::MGGA_C_VSXC => "MGGA_C_VSXC",
            XcFuncId::MGGA_C_M06_L => "MGGA_C_M06_L",
            XcFuncId::MGGA_C_M06_HF => "MGGA_C_M06_HF",
            XcFuncId::MGGA_C_M06 => "MGGA_C_M06",
            XcFuncId::MGGA_C_M06_2X => "MGGA_C_M06_2X",
            XcFuncId::MGGA_C_M05 => "MGGA_C_M05",
            XcFuncId::MGGA_C_M05_2X => "MGGA_C_M05_2X",
            XcFuncId::MGGA_C_PKZB => "MGGA_C_PKZB",
            XcFuncId::MGGA_C_BC95 => "MGGA_C_BC95",
            XcFuncId::MGGA_C_REVTPSS => "MGGA_C_REVTPSS",
            XcFuncId::MGGA_XC_TPSSLYP1W => "MGGA_XC_TPSSLYP1W",
            XcFuncId::MGGA_X_MK00B => "MGGA_X_MK00B",
            XcFuncId::MGGA_X_BLOC => "MGGA_X_BLOC",
            XcFuncId::MGGA_X_MODTPSS => "MGGA_X_MODTPSS",
            XcFuncId::GGA_C_PBELOC => "GGA_C_PBELOC",
            XcFuncId::MGGA_C_TPSSLOC => "MGGA_C_TPSSLOC",
            XcFuncId::HYB_MGGA_X_MN12_SX => "HYB_MGGA_X_MN12_SX",
            XcFuncId::MGGA_X_MBEEF => "MGGA_X_MBEEF",
            XcFuncId::MGGA_X_MBEEFVDW => "MGGA_X_MBEEFVDW",
            XcFuncId::MGGA_C_TM => "MGGA_C_TM",
            XcFuncId::GGA_C_P86VWN => "GGA_C_P86VWN",
            XcFuncId::GGA_C_P86VWN_FT => "GGA_C_P86VWN_FT",
            XcFuncId::MGGA_XC_B97M_V => "MGGA_XC_B97M_V",
            XcFuncId::GGA_XC_VV10 => "GGA_XC_VV10",
            XcFuncId::MGGA_X_JK => "MGGA_X_JK",
            XcFuncId::MGGA_X_MVS => "MGGA_X_MVS",
            XcFuncId::GGA_C_PBEFE => "GGA_C_PBEFE",
            XcFuncId::LDA_XC_KSDT => "LDA_XC_KSDT",
            XcFuncId::MGGA_X_MN15_L => "MGGA_X_MN15_L",
            XcFuncId::MGGA_C_MN15_L => "MGGA_C_MN15_L",
            XcFuncId::GGA_C_OP_PW91 => "GGA_C_OP_PW91",
            XcFuncId::MGGA_X_SCAN => "MGGA_X_SCAN",
            XcFuncId::HYB_MGGA_X_SCAN0 => "HYB_MGGA_X_SCAN0",
            XcFuncId::GGA_X_PBEFE => "GGA_X_PBEFE",
            XcFuncId::HYB_GGA_XC_B97_1P => "HYB_GGA_XC_B97_1P",
            XcFuncId::MGGA_C_SCAN => "MGGA_C_SCAN",
            XcFuncId::HYB_MGGA_X_MN15 => "HYB_MGGA_X_MN15",
            XcFuncId::MGGA_C_MN15 => "MGGA_C_MN15",
            XcFuncId::GGA_X_CAP => "GGA_X_CAP",
            XcFuncId::GGA_X_EB88 => "GGA_X_EB88",
            XcFuncId::GGA_C_PBE_MOL => "GGA_C_PBE_MOL",
            XcFuncId::HYB_GGA_XC_PBE_MOL0 => "HYB_GGA_XC_PBE_MOL0",
            XcFuncId::HYB_GGA_XC_PBE_SOL0 => "HYB_GGA_XC_PBE_SOL0",
            XcFuncId::HYB_GGA_XC_PBEB0 => "HYB_GGA_XC_PBEB0",
            XcFuncId::HYB_GGA_XC_PBE_MOLB0 => "HYB_GGA_XC_PBE_MOLB0",
            XcFuncId::GGA_K_ABSP3 => "GGA_K_ABSP3",
            XcFuncId::GGA_K_ABSP4 => "GGA_K_ABSP4",
            XcFuncId::HYB_MGGA_X_BMK => "HYB_MGGA_X_BMK",
            XcFuncId::GGA_C_BMK => "GGA_C_BMK",
            XcFuncId::GGA_C_TAU_HCTH => "GGA_C_TAU_HCTH",
            XcFuncId::HYB_MGGA_X_TAU_HCTH => "HYB_MGGA_X_TAU_HCTH",
            XcFuncId::GGA_C_HYB_TAU_HCTH => "GGA_C_HYB_TAU_HCTH",
            XcFuncId::MGGA_X_B00 => "MGGA_X_B00",
            XcFuncId::GGA_X_BEEFVDW => "GGA_X_BEEFVDW",
            XcFuncId::GGA_XC_BEEFVDW => "GGA_XC_BEEFVDW",
            XcFuncId::LDA_C_CHACHIYO => "LDA_C_CHACHIYO",
            XcFuncId::MGGA_XC_HLE17 => "MGGA_XC_HLE17",
            XcFuncId::LDA_C_LP96 => "LDA_C_LP96",
            XcFuncId::HYB_GGA_XC_PBE50 => "HYB_GGA_XC_PBE50",
            XcFuncId::GGA_X_PBETRANS => "GGA_X_PBETRANS",
            XcFuncId::MGGA_C_SCAN_RVV10 => "MGGA_C_SCAN_RVV10",
            XcFuncId::MGGA_X_REVM06_L => "MGGA_X_REVM06_L",
            XcFuncId::MGGA_C_REVM06_L => "MGGA_C_REVM06_L",
            XcFuncId::HYB_MGGA_X_M08_HX => "HYB_MGGA_X_M08_HX",
            XcFuncId::HYB_MGGA_X_M08_SO => "HYB_MGGA_X_M08_SO",
            XcFuncId::HYB_MGGA_X_M11 => "HYB_MGGA_X_M11",
            XcFuncId::GGA_X_CHACHIYO => "GGA_X_CHACHIYO",
            XcFuncId::MGGA_X_RTPSS => "MGGA_X_RTPSS",
            XcFuncId::MGGA_X_MS2B => "MGGA_X_MS2B",
            XcFuncId::MGGA_X_MS2BS => "MGGA_X_MS2BS",
            XcFuncId::MGGA_X_MVSB => "MGGA_X_MVSB",
            XcFuncId::MGGA_X_MVSBS => "MGGA_X_MVSBS",
            XcFuncId::HYB_MGGA_X_REVM11 => "HYB_MGGA_X_REVM11",
            XcFuncId::HYB_MGGA_X_REVM06 => "HYB_MGGA_X_REVM06",
            XcFuncId::MGGA_C_REVM06 => "MGGA_C_REVM06",
            XcFuncId::LDA_C_CHACHIYO_MOD => "LDA_C_CHACHIYO_MOD",
            XcFuncId::LDA_C_KARASIEV_MOD => "LDA_C_KARASIEV_MOD",
            XcFuncId::GGA_C_CHACHIYO => "GGA_C_CHACHIYO",
            XcFuncId::HYB_MGGA_X_M06_SX => "HYB_MGGA_X_M06_SX",
            XcFuncId::MGGA_C_M06_SX => "MGGA_C_M06_SX",
            XcFuncId::GGA_X_REVSSB_D => "GGA_X_REVSSB_D",
            XcFuncId::GGA_C_CCDF => "GGA_C_CCDF",
            XcFuncId::HYB_GGA_XC_HFLYP => "HYB_GGA_XC_HFLYP",
            XcFuncId::HYB_GGA_XC_B3P86_NWCHEM => "HYB_GGA_XC_B3P86_NWCHEM",
            XcFuncId::GGA_X_PW91_MOD => "GGA_X_PW91_MOD",
            XcFuncId::LDA_C_W20 => "LDA_C_W20",
            XcFuncId::LDA_XC_CORRKSDT => "LDA_XC_CORRKSDT",
            XcFuncId::MGGA_X_FT98 => "MGGA_X_FT98",
            XcFuncId::GGA_X_PBE_MOD => "GGA_X_PBE_MOD",
            XcFuncId::GGA_X_PBE_GAUSSIAN => "GGA_X_PBE_GAUSSIAN",
            XcFuncId::GGA_C_PBE_GAUSSIAN => "GGA_C_PBE_GAUSSIAN",
            XcFuncId::MGGA_C_TPSS_GAUSSIAN => "MGGA_C_TPSS_GAUSSIAN",
            XcFuncId::GGA_X_NCAPR => "GGA_X_NCAPR",
            XcFuncId::HYB_GGA_XC_RELPBE0 => "HYB_GGA_XC_RELPBE0",
            XcFuncId::MGGA_X_EEL => "MGGA_X_EEL",
            XcFuncId::GGA_XC_B97_3C => "GGA_XC_B97_3C",
            XcFuncId::LDA_C_EPC17 => "LDA_C_EPC17",
            XcFuncId::LDA_C_EPC17_2 => "LDA_C_EPC17_2",
            XcFuncId::LDA_C_EPC18_1 => "LDA_C_EPC18_1",
            XcFuncId::LDA_C_EPC18_2 => "LDA_C_EPC18_2",
            XcFuncId::GGA_XC_DLB97 => "GGA_XC_DLB97",
            XcFuncId::MGGA_X_MSCAN => "MGGA_X_MSCAN",
            XcFuncId::MGGA_C_MSCAN => "MGGA_C_MSCAN",
            XcFuncId::GGA_X_T_PBE1 => "GGA_X_T_PBE1",
            XcFuncId::GGA_X_T_PBE2 => "GGA_X_T_PBE2",
            XcFuncId::LDA_X_T_SLOC => "LDA_X_T_SLOC",
            XcFuncId::GGA_X_BKL1 => "GGA_X_BKL1",
            XcFuncId::GGA_X_BKL2 => "GGA_X_BKL2",
            XcFuncId::HYB_MGGA_X_CF22D => "HYB_MGGA_X_CF22D",
            XcFuncId::MGGA_C_CF22D => "MGGA_C_CF22D",
            XcFuncId::MGGA_X_LAK => "MGGA_X_LAK",
            XcFuncId::GGA_C_BKL1 => "GGA_C_BKL1",
            XcFuncId::GGA_C_BKL2 => "GGA_C_BKL2",
            XcFuncId::MGGA_C_LAK => "MGGA_C_LAK",
            XcFuncId::GGA_X_DF3_OPT1 => "GGA_X_DF3_OPT1",
            XcFuncId::GGA_X_DF3_OPT2 => "GGA_X_DF3_OPT2",
            XcFuncId::HYB_GGA_XC_CQTP25 => "HYB_GGA_XC_CQTP25",
            XcFuncId::HYB_GGA_XC_OPB3LYP => "HYB_GGA_XC_OPB3LYP",
            XcFuncId::MGGA_C_CC => "MGGA_C_CC",
            XcFuncId::MGGA_C_CCALDA => "MGGA_C_CCALDA",
            XcFuncId::HYB_MGGA_XC_BR3P86 => "HYB_MGGA_XC_BR3P86",
            XcFuncId::HYB_GGA_XC_CASE21 => "HYB_GGA_XC_CASE21",
            XcFuncId::MGGA_C_RREGTM => "MGGA_C_RREGTM",
            XcFuncId::HYB_GGA_XC_PBE_2X => "HYB_GGA_XC_PBE_2X",
            XcFuncId::HYB_GGA_XC_PBE38 => "HYB_GGA_XC_PBE38",
            XcFuncId::HYB_GGA_XC_B3LYP3 => "HYB_GGA_XC_B3LYP3",
            XcFuncId::HYB_GGA_XC_CAM_O3LYP => "HYB_GGA_XC_CAM_O3LYP",
            XcFuncId::HYB_MGGA_XC_TPSS0 => "HYB_MGGA_XC_TPSS0",
            XcFuncId::MGGA_C_B94 => "MGGA_C_B94",
            XcFuncId::HYB_MGGA_XC_B94_HYB => "HYB_MGGA_XC_B94_HYB",
            XcFuncId::HYB_GGA_XC_WB97X_D3 => "HYB_GGA_XC_WB97X_D3",
            XcFuncId::HYB_GGA_XC_LC_BLYP => "HYB_GGA_XC_LC_BLYP",
            XcFuncId::HYB_GGA_XC_B3PW91 => "HYB_GGA_XC_B3PW91",
            XcFuncId::HYB_GGA_XC_B3LYP => "HYB_GGA_XC_B3LYP",
            XcFuncId::HYB_GGA_XC_B3P86 => "HYB_GGA_XC_B3P86",
            XcFuncId::HYB_GGA_XC_O3LYP => "HYB_GGA_XC_O3LYP",
            XcFuncId::HYB_GGA_XC_MPW1K => "HYB_GGA_XC_MPW1K",
            XcFuncId::HYB_GGA_XC_PBEH => "HYB_GGA_XC_PBEH",
            XcFuncId::HYB_GGA_XC_B97 => "HYB_GGA_XC_B97",
            XcFuncId::HYB_GGA_XC_B97_1 => "HYB_GGA_XC_B97_1",
            XcFuncId::HYB_GGA_XC_APF => "HYB_GGA_XC_APF",
            XcFuncId::HYB_GGA_XC_B97_2 => "HYB_GGA_XC_B97_2",
            XcFuncId::HYB_GGA_XC_X3LYP => "HYB_GGA_XC_X3LYP",
            XcFuncId::HYB_GGA_XC_B1WC => "HYB_GGA_XC_B1WC",
            XcFuncId::HYB_GGA_XC_B97_K => "HYB_GGA_XC_B97_K",
            XcFuncId::HYB_GGA_XC_B97_3 => "HYB_GGA_XC_B97_3",
            XcFuncId::HYB_GGA_XC_MPW3PW => "HYB_GGA_XC_MPW3PW",
            XcFuncId::HYB_GGA_XC_B1LYP => "HYB_GGA_XC_B1LYP",
            XcFuncId::HYB_GGA_XC_B1PW91 => "HYB_GGA_XC_B1PW91",
            XcFuncId::HYB_GGA_XC_MPW1PW => "HYB_GGA_XC_MPW1PW",
            XcFuncId::HYB_GGA_XC_MPW3LYP => "HYB_GGA_XC_MPW3LYP",
            XcFuncId::HYB_GGA_XC_SB98_1A => "HYB_GGA_XC_SB98_1A",
            XcFuncId::HYB_GGA_XC_SB98_1B => "HYB_GGA_XC_SB98_1B",
            XcFuncId::HYB_GGA_XC_SB98_1C => "HYB_GGA_XC_SB98_1C",
            XcFuncId::HYB_GGA_XC_SB98_2A => "HYB_GGA_XC_SB98_2A",
            XcFuncId::HYB_GGA_XC_SB98_2B => "HYB_GGA_XC_SB98_2B",
            XcFuncId::HYB_GGA_XC_SB98_2C => "HYB_GGA_XC_SB98_2C",
            XcFuncId::HYB_GGA_X_SOGGA11_X => "HYB_GGA_X_SOGGA11_X",
            XcFuncId::HYB_GGA_XC_HSE03 => "HYB_GGA_XC_HSE03",
            XcFuncId::HYB_GGA_XC_HSE06 => "HYB_GGA_XC_HSE06",
            XcFuncId::HYB_GGA_XC_HJS_PBE => "HYB_GGA_XC_HJS_PBE",
            XcFuncId::HYB_GGA_XC_HJS_PBE_SOL => "HYB_GGA_XC_HJS_PBE_SOL",
            XcFuncId::HYB_GGA_XC_HJS_B88 => "HYB_GGA_XC_HJS_B88",
            XcFuncId::HYB_GGA_XC_HJS_B97X => "HYB_GGA_XC_HJS_B97X",
            XcFuncId::HYB_GGA_XC_CAM_B3LYP => "HYB_GGA_XC_CAM_B3LYP",
            XcFuncId::HYB_GGA_XC_TUNED_CAM_B3LYP => "HYB_GGA_XC_TUNED_CAM_B3LYP",
            XcFuncId::HYB_GGA_XC_BHANDH => "HYB_GGA_XC_BHANDH",
            XcFuncId::HYB_GGA_XC_BHANDHLYP => "HYB_GGA_XC_BHANDHLYP",
            XcFuncId::HYB_GGA_XC_MB3LYP_RC04 => "HYB_GGA_XC_MB3LYP_RC04",
            XcFuncId::HYB_MGGA_X_M05 => "HYB_MGGA_X_M05",
            XcFuncId::HYB_MGGA_X_M05_2X => "HYB_MGGA_X_M05_2X",
            XcFuncId::HYB_MGGA_XC_B88B95 => "HYB_MGGA_XC_B88B95",
            XcFuncId::HYB_MGGA_XC_B86B95 => "HYB_MGGA_XC_B86B95",
            XcFuncId::HYB_MGGA_XC_PW86B95 => "HYB_MGGA_XC_PW86B95",
            XcFuncId::HYB_MGGA_XC_BB1K => "HYB_MGGA_XC_BB1K",
            XcFuncId::HYB_MGGA_X_M06_HF => "HYB_MGGA_X_M06_HF",
            XcFuncId::HYB_MGGA_XC_MPW1B95 => "HYB_MGGA_XC_MPW1B95",
            XcFuncId::HYB_MGGA_XC_MPWB1K => "HYB_MGGA_XC_MPWB1K",
            XcFuncId::HYB_MGGA_XC_X1B95 => "HYB_MGGA_XC_X1B95",
            XcFuncId::HYB_MGGA_XC_XB1K => "HYB_MGGA_XC_XB1K",
            XcFuncId::HYB_MGGA_X_M06 => "HYB_MGGA_X_M06",
            XcFuncId::HYB_MGGA_X_M06_2X => "HYB_MGGA_X_M06_2X",
            XcFuncId::HYB_MGGA_XC_PW6B95 => "HYB_MGGA_XC_PW6B95",
            XcFuncId::HYB_MGGA_XC_PWB6K => "HYB_MGGA_XC_PWB6K",
            XcFuncId::HYB_GGA_XC_MPWLYP1M => "HYB_GGA_XC_MPWLYP1M",
            XcFuncId::HYB_GGA_XC_REVB3LYP => "HYB_GGA_XC_REVB3LYP",
            XcFuncId::HYB_GGA_XC_CAMY_BLYP => "HYB_GGA_XC_CAMY_BLYP",
            XcFuncId::HYB_GGA_XC_PBE0_13 => "HYB_GGA_XC_PBE0_13",
            XcFuncId::HYB_MGGA_XC_TPSSH => "HYB_MGGA_XC_TPSSH",
            XcFuncId::HYB_MGGA_XC_REVTPSSH => "HYB_MGGA_XC_REVTPSSH",
            XcFuncId::HYB_GGA_XC_B3LYPS => "HYB_GGA_XC_B3LYPS",
            XcFuncId::HYB_GGA_XC_QTP17 => "HYB_GGA_XC_QTP17",
            XcFuncId::HYB_GGA_XC_B3LYP_MCM1 => "HYB_GGA_XC_B3LYP_MCM1",
            XcFuncId::HYB_GGA_XC_B3LYP_MCM2 => "HYB_GGA_XC_B3LYP_MCM2",
            XcFuncId::HYB_GGA_XC_WB97 => "HYB_GGA_XC_WB97",
            XcFuncId::HYB_GGA_XC_WB97X => "HYB_GGA_XC_WB97X",
            XcFuncId::HYB_GGA_XC_LRC_WPBEH => "HYB_GGA_XC_LRC_WPBEH",
            XcFuncId::HYB_GGA_XC_WB97X_V => "HYB_GGA_XC_WB97X_V",
            XcFuncId::HYB_GGA_XC_LCY_PBE => "HYB_GGA_XC_LCY_PBE",
            XcFuncId::HYB_GGA_XC_LCY_BLYP => "HYB_GGA_XC_LCY_BLYP",
            XcFuncId::HYB_GGA_XC_LC_VV10 => "HYB_GGA_XC_LC_VV10",
            XcFuncId::HYB_GGA_XC_CAMY_B3LYP => "HYB_GGA_XC_CAMY_B3LYP",
            XcFuncId::HYB_GGA_XC_WB97X_D => "HYB_GGA_XC_WB97X_D",
            XcFuncId::HYB_GGA_XC_HPBEINT => "HYB_GGA_XC_HPBEINT",
            XcFuncId::HYB_GGA_XC_LRC_WPBE => "HYB_GGA_XC_LRC_WPBE",
            XcFuncId::HYB_MGGA_X_MVSH => "HYB_MGGA_X_MVSH",
            XcFuncId::HYB_GGA_XC_B3LYP5 => "HYB_GGA_XC_B3LYP5",
            XcFuncId::HYB_GGA_XC_EDF2 => "HYB_GGA_XC_EDF2",
            XcFuncId::HYB_GGA_XC_CAP0 => "HYB_GGA_XC_CAP0",
            XcFuncId::HYB_GGA_XC_LC_WPBE => "HYB_GGA_XC_LC_WPBE",
            XcFuncId::HYB_GGA_XC_HSE12 => "HYB_GGA_XC_HSE12",
            XcFuncId::HYB_GGA_XC_HSE12S => "HYB_GGA_XC_HSE12S",
            XcFuncId::HYB_GGA_XC_HSE_SOL => "HYB_GGA_XC_HSE_SOL",
            XcFuncId::HYB_GGA_XC_CAM_QTP_01 => "HYB_GGA_XC_CAM_QTP_01",
            XcFuncId::HYB_GGA_XC_MPW1LYP => "HYB_GGA_XC_MPW1LYP",
            XcFuncId::HYB_GGA_XC_MPW1PBE => "HYB_GGA_XC_MPW1PBE",
            XcFuncId::HYB_GGA_XC_KMLYP => "HYB_GGA_XC_KMLYP",
            XcFuncId::HYB_GGA_XC_LC_WPBE_WHS => "HYB_GGA_XC_LC_WPBE_WHS",
            XcFuncId::HYB_GGA_XC_LC_WPBEH_WHS => "HYB_GGA_XC_LC_WPBEH_WHS",
            XcFuncId::HYB_GGA_XC_LC_WPBE08_WHS => "HYB_GGA_XC_LC_WPBE08_WHS",
            XcFuncId::HYB_GGA_XC_LC_WPBESOL_WHS => "HYB_GGA_XC_LC_WPBESOL_WHS",
            XcFuncId::HYB_GGA_XC_CAM_QTP_00 => "HYB_GGA_XC_CAM_QTP_00",
            XcFuncId::HYB_GGA_XC_CAM_QTP_02 => "HYB_GGA_XC_CAM_QTP_02",
            XcFuncId::HYB_GGA_XC_LC_QTP => "HYB_GGA_XC_LC_QTP",
            XcFuncId::MGGA_X_RSCAN => "MGGA_X_RSCAN",
            XcFuncId::MGGA_C_RSCAN => "MGGA_C_RSCAN",
            XcFuncId::GGA_X_S12G => "GGA_X_S12G",
            XcFuncId::HYB_GGA_X_S12H => "HYB_GGA_X_S12H",
            XcFuncId::MGGA_X_R2SCAN => "MGGA_X_R2SCAN",
            XcFuncId::MGGA_C_R2SCAN => "MGGA_C_R2SCAN",
            XcFuncId::HYB_GGA_XC_BLYP35 => "HYB_GGA_XC_BLYP35",
            XcFuncId::GGA_K_VW => "GGA_K_VW",
            XcFuncId::GGA_K_GE2 => "GGA_K_GE2",
            XcFuncId::GGA_K_GOLDEN => "GGA_K_GOLDEN",
            XcFuncId::GGA_K_YT65 => "GGA_K_YT65",
            XcFuncId::GGA_K_BALTIN => "GGA_K_BALTIN",
            XcFuncId::GGA_K_LIEB => "GGA_K_LIEB",
            XcFuncId::GGA_K_ABSP1 => "GGA_K_ABSP1",
            XcFuncId::GGA_K_ABSP2 => "GGA_K_ABSP2",
            XcFuncId::GGA_K_GR => "GGA_K_GR",
            XcFuncId::GGA_K_LUDENA => "GGA_K_LUDENA",
            XcFuncId::GGA_K_GP85 => "GGA_K_GP85",
            XcFuncId::GGA_K_PEARSON => "GGA_K_PEARSON",
            XcFuncId::GGA_K_OL1 => "GGA_K_OL1",
            XcFuncId::GGA_K_OL2 => "GGA_K_OL2",
            XcFuncId::GGA_K_FR_B88 => "GGA_K_FR_B88",
            XcFuncId::GGA_K_FR_PW86 => "GGA_K_FR_PW86",
            XcFuncId::GGA_K_DK => "GGA_K_DK",
            XcFuncId::GGA_K_PERDEW => "GGA_K_PERDEW",
            XcFuncId::GGA_K_VSK => "GGA_K_VSK",
            XcFuncId::GGA_K_VJKS => "GGA_K_VJKS",
            XcFuncId::GGA_K_ERNZERHOF => "GGA_K_ERNZERHOF",
            XcFuncId::GGA_K_LC94 => "GGA_K_LC94",
            XcFuncId::GGA_K_LLP => "GGA_K_LLP",
            XcFuncId::GGA_K_THAKKAR => "GGA_K_THAKKAR",
            XcFuncId::GGA_X_WPBEH => "GGA_X_WPBEH",
            XcFuncId::GGA_X_HJS_PBE => "GGA_X_HJS_PBE",
            XcFuncId::GGA_X_HJS_PBE_SOL => "GGA_X_HJS_PBE_SOL",
            XcFuncId::GGA_X_HJS_B88 => "GGA_X_HJS_B88",
            XcFuncId::GGA_X_HJS_B97X => "GGA_X_HJS_B97X",
            XcFuncId::GGA_X_ITYH => "GGA_X_ITYH",
            XcFuncId::GGA_X_SFAT => "GGA_X_SFAT",
            XcFuncId::HYB_MGGA_XC_WB97M_V => "HYB_MGGA_XC_WB97M_V",
            XcFuncId::LDA_X_REL => "LDA_X_REL",
            XcFuncId::GGA_X_SG4 => "GGA_X_SG4",
            XcFuncId::GGA_C_SG4 => "GGA_C_SG4",
            XcFuncId::GGA_X_GG99 => "GGA_X_GG99",
            XcFuncId::LDA_XC_1D_EHWLRG_1 => "LDA_XC_1D_EHWLRG_1",
            XcFuncId::LDA_XC_1D_EHWLRG_2 => "LDA_XC_1D_EHWLRG_2",
            XcFuncId::LDA_XC_1D_EHWLRG_3 => "LDA_XC_1D_EHWLRG_3",
            XcFuncId::GGA_X_PBEPOW => "GGA_X_PBEPOW",
            XcFuncId::MGGA_X_TM => "MGGA_X_TM",
            XcFuncId::MGGA_X_VT84 => "MGGA_X_VT84",
            XcFuncId::MGGA_X_SA_TPSS => "MGGA_X_SA_TPSS",
            XcFuncId::MGGA_K_PC07 => "MGGA_K_PC07",
            XcFuncId::GGA_X_KGG99 => "GGA_X_KGG99",
            XcFuncId::GGA_XC_HLE16 => "GGA_XC_HLE16",
            XcFuncId::LDA_X_ERF => "LDA_X_ERF",
            XcFuncId::LDA_XC_LP_A => "LDA_XC_LP_A",
            XcFuncId::LDA_XC_LP_B => "LDA_XC_LP_B",
            XcFuncId::LDA_X_RAE => "LDA_X_RAE",
            XcFuncId::LDA_K_ZLP => "LDA_K_ZLP",
            XcFuncId::LDA_C_MCWEENY => "LDA_C_MCWEENY",
            XcFuncId::LDA_C_BR78 => "LDA_C_BR78",
            XcFuncId::GGA_C_SCAN_E0 => "GGA_C_SCAN_E0",
            XcFuncId::LDA_C_PK09 => "LDA_C_PK09",
            XcFuncId::GGA_C_GAPC => "GGA_C_GAPC",
            XcFuncId::GGA_C_GAPLOC => "GGA_C_GAPLOC",
            XcFuncId::GGA_C_ZVPBEINT => "GGA_C_ZVPBEINT",
            XcFuncId::GGA_C_ZVPBESOL => "GGA_C_ZVPBESOL",
            XcFuncId::GGA_C_TM_LYP => "GGA_C_TM_LYP",
            XcFuncId::GGA_C_TM_PBE => "GGA_C_TM_PBE",
            XcFuncId::GGA_C_W94 => "GGA_C_W94",
            XcFuncId::MGGA_C_KCIS => "MGGA_C_KCIS",
            XcFuncId::HYB_MGGA_XC_B0KCIS => "HYB_MGGA_XC_B0KCIS",
            XcFuncId::MGGA_XC_LP90 => "MGGA_XC_LP90",
            XcFuncId::GGA_C_CS1 => "GGA_C_CS1",
            XcFuncId::HYB_MGGA_XC_MPW1KCIS => "HYB_MGGA_XC_MPW1KCIS",
            XcFuncId::HYB_MGGA_XC_MPWKCIS1K => "HYB_MGGA_XC_MPWKCIS1K",
            XcFuncId::HYB_MGGA_XC_PBE1KCIS => "HYB_MGGA_XC_PBE1KCIS",
            XcFuncId::HYB_MGGA_XC_TPSS1KCIS => "HYB_MGGA_XC_TPSS1KCIS",
            XcFuncId::GGA_X_B88M => "GGA_X_B88M",
            XcFuncId::MGGA_C_B88 => "MGGA_C_B88",
            XcFuncId::HYB_GGA_XC_B5050LYP => "HYB_GGA_XC_B5050LYP",
            XcFuncId::LDA_C_OW_LYP => "LDA_C_OW_LYP",
            XcFuncId::LDA_C_OW => "LDA_C_OW",
            XcFuncId::MGGA_X_GX => "MGGA_X_GX",
            XcFuncId::MGGA_X_PBE_GX => "MGGA_X_PBE_GX",
            XcFuncId::LDA_XC_GDSMFB => "LDA_XC_GDSMFB",
            XcFuncId::LDA_C_GK72 => "LDA_C_GK72",
            XcFuncId::LDA_C_KARASIEV => "LDA_C_KARASIEV",
            XcFuncId::LDA_K_LP96 => "LDA_K_LP96",
            XcFuncId::MGGA_X_REVSCAN => "MGGA_X_REVSCAN",
            XcFuncId::MGGA_C_REVSCAN => "MGGA_C_REVSCAN",
            XcFuncId::HYB_MGGA_X_REVSCAN0 => "HYB_MGGA_X_REVSCAN0",
            XcFuncId::MGGA_C_SCAN_VV10 => "MGGA_C_SCAN_VV10",
            XcFuncId::MGGA_C_REVSCAN_VV10 => "MGGA_C_REVSCAN_VV10",
            XcFuncId::MGGA_X_BR89_EXPLICIT => "MGGA_X_BR89_EXPLICIT",
            XcFuncId::GGA_XC_KT3 => "GGA_XC_KT3",
            XcFuncId::HYB_LDA_XC_BN05 => "HYB_LDA_XC_BN05",
            XcFuncId::HYB_GGA_XC_LB07 => "HYB_GGA_XC_LB07",
            XcFuncId::LDA_C_PMGB06 => "LDA_C_PMGB06",
            XcFuncId::GGA_K_GDS08 => "GGA_K_GDS08",
            XcFuncId::GGA_K_GHDS10 => "GGA_K_GHDS10",
            XcFuncId::GGA_K_GHDS10R => "GGA_K_GHDS10R",
            XcFuncId::GGA_K_TKVLN => "GGA_K_TKVLN",
            XcFuncId::GGA_K_PBE3 => "GGA_K_PBE3",
            XcFuncId::GGA_K_PBE4 => "GGA_K_PBE4",
            XcFuncId::GGA_K_EXP4 => "GGA_K_EXP4",
            XcFuncId::HYB_MGGA_XC_B98 => "HYB_MGGA_XC_B98",
            XcFuncId::LDA_XC_TIH => "LDA_XC_TIH",
            XcFuncId::LDA_X_1D_EXPONENTIAL => "LDA_X_1D_EXPONENTIAL",
            XcFuncId::GGA_X_SFAT_PBE => "GGA_X_SFAT_PBE",
            XcFuncId::MGGA_X_BR89_EXPLICIT_1 => "MGGA_X_BR89_EXPLICIT_1",
            XcFuncId::MGGA_X_REGTPSS => "MGGA_X_REGTPSS",
            XcFuncId::GGA_X_FD_LB94 => "GGA_X_FD_LB94",
            XcFuncId::GGA_X_FD_REVLB94 => "GGA_X_FD_REVLB94",
            XcFuncId::GGA_C_ZVPBELOC => "GGA_C_ZVPBELOC",
            XcFuncId::HYB_GGA_XC_APBE0 => "HYB_GGA_XC_APBE0",
            XcFuncId::HYB_GGA_XC_HAPBE => "HYB_GGA_XC_HAPBE",
            XcFuncId::MGGA_X_2D_JS17 => "MGGA_X_2D_JS17",
            XcFuncId::HYB_GGA_XC_RCAM_B3LYP => "HYB_GGA_XC_RCAM_B3LYP",
            XcFuncId::HYB_GGA_XC_WC04 => "HYB_GGA_XC_WC04",
            XcFuncId::HYB_GGA_XC_WP04 => "HYB_GGA_XC_WP04",
            XcFuncId::GGA_K_LKT => "GGA_K_LKT",
            XcFuncId::HYB_GGA_XC_CAMH_B3LYP => "HYB_GGA_XC_CAMH_B3LYP",
            XcFuncId::HYB_GGA_XC_WHPBE0 => "HYB_GGA_XC_WHPBE0",
            XcFuncId::GGA_K_PBE2 => "GGA_K_PBE2",
            XcFuncId::MGGA_K_L04 => "MGGA_K_L04",
            XcFuncId::MGGA_K_L06 => "MGGA_K_L06",
            XcFuncId::GGA_K_VT84F => "GGA_K_VT84F",
            XcFuncId::GGA_K_LGAP => "GGA_K_LGAP",
            XcFuncId::MGGA_K_RDA => "MGGA_K_RDA",
            XcFuncId::GGA_X_ITYH_OPTX => "GGA_X_ITYH_OPTX",
            XcFuncId::GGA_X_ITYH_PBE => "GGA_X_ITYH_PBE",
            XcFuncId::GGA_C_LYPR => "GGA_C_LYPR",
            XcFuncId::HYB_GGA_XC_LC_BLYP_EA => "HYB_GGA_XC_LC_BLYP_EA",
            XcFuncId::MGGA_X_REGTM => "MGGA_X_REGTM",
            XcFuncId::MGGA_K_GEA2 => "MGGA_K_GEA2",
            XcFuncId::MGGA_K_GEA4 => "MGGA_K_GEA4",
            XcFuncId::MGGA_K_CSK1 => "MGGA_K_CSK1",
            XcFuncId::MGGA_K_CSK4 => "MGGA_K_CSK4",
            XcFuncId::MGGA_K_CSK_LOC1 => "MGGA_K_CSK_LOC1",
            XcFuncId::MGGA_K_CSK_LOC4 => "MGGA_K_CSK_LOC4",
            XcFuncId::GGA_K_LGAP_GE => "GGA_K_LGAP_GE",
            XcFuncId::MGGA_K_PC07_OPT => "MGGA_K_PC07_OPT",
            XcFuncId::GGA_K_TFVW_OPT => "GGA_K_TFVW_OPT",
            XcFuncId::HYB_GGA_XC_LC_BOP => "HYB_GGA_XC_LC_BOP",
            XcFuncId::HYB_GGA_XC_LC_PBEOP => "HYB_GGA_XC_LC_PBEOP",
            XcFuncId::MGGA_C_KCISK => "MGGA_C_KCISK",
            XcFuncId::HYB_GGA_XC_LC_BLYPR => "HYB_GGA_XC_LC_BLYPR",
            XcFuncId::HYB_GGA_XC_MCAM_B3LYP => "HYB_GGA_XC_MCAM_B3LYP",
            XcFuncId::LDA_X_YUKAWA => "LDA_X_YUKAWA",
            XcFuncId::MGGA_C_R2SCAN01 => "MGGA_C_R2SCAN01",
            XcFuncId::MGGA_C_RMGGAC => "MGGA_C_RMGGAC",
            XcFuncId::MGGA_X_MCML => "MGGA_X_MCML",
            XcFuncId::MGGA_X_R2SCAN01 => "MGGA_X_R2SCAN01",
            XcFuncId::HYB_GGA_X_CAM_S12G => "HYB_GGA_X_CAM_S12G",
            XcFuncId::HYB_GGA_X_CAM_S12H => "HYB_GGA_X_CAM_S12H",
            XcFuncId::MGGA_X_RPPSCAN => "MGGA_X_RPPSCAN",
            XcFuncId::MGGA_C_RPPSCAN => "MGGA_C_RPPSCAN",
            XcFuncId::MGGA_X_R4SCAN => "MGGA_X_R4SCAN",
            XcFuncId::MGGA_X_VCML => "MGGA_X_VCML",
            XcFuncId::MGGA_XC_VCML_RVV10 => "MGGA_XC_VCML_RVV10",
            XcFuncId::HYB_LDA_X_ERF => "HYB_LDA_X_ERF",
            XcFuncId::LDA_C_PW_ERF => "LDA_C_PW_ERF",
            XcFuncId::GGA_X_PBE_ERF_GWS => "GGA_X_PBE_ERF_GWS",
            XcFuncId::HYB_GGA_X_PBE_ERF_GWS => "HYB_GGA_X_PBE_ERF_GWS",
            XcFuncId::GGA_C_PBE_ERF_GWS => "GGA_C_PBE_ERF_GWS",
            XcFuncId::HYB_MGGA_XC_GAS22 => "HYB_MGGA_XC_GAS22",
            XcFuncId::HYB_MGGA_XC_R2SCANH => "HYB_MGGA_XC_R2SCANH",
            XcFuncId::HYB_MGGA_XC_R2SCAN0 => "HYB_MGGA_XC_R2SCAN0",
            XcFuncId::HYB_MGGA_XC_R2SCAN50 => "HYB_MGGA_XC_R2SCAN50",
            XcFuncId::HYB_MGGA_X_WR2SCAN => "HYB_MGGA_X_WR2SCAN",
            XcFuncId::HYB_GGA_XC_CAM_PBEH => "HYB_GGA_XC_CAM_PBEH",
            XcFuncId::HYB_GGA_XC_CAMY_PBEH => "HYB_GGA_XC_CAMY_PBEH",
            XcFuncId::LDA_C_UPW92 => "LDA_C_UPW92",
            XcFuncId::LDA_C_RPW92 => "LDA_C_RPW92",
            XcFuncId::MGGA_X_TLDA => "MGGA_X_TLDA",
            XcFuncId::MGGA_X_EDMGGA => "MGGA_X_EDMGGA",
            XcFuncId::MGGA_X_GDME_NV => "MGGA_X_GDME_NV",
            XcFuncId::MGGA_X_RLDA => "MGGA_X_RLDA",
            XcFuncId::MGGA_X_GDME_0 => "MGGA_X_GDME_0",
            XcFuncId::MGGA_X_GDME_KOS => "MGGA_X_GDME_KOS",
            XcFuncId::MGGA_X_GDME_VT => "MGGA_X_GDME_VT",
            XcFuncId::LDA_X_SLOC => "LDA_X_SLOC",
            XcFuncId::MGGA_X_REVTM => "MGGA_X_REVTM",
            XcFuncId::MGGA_C_REVTM => "MGGA_C_REVTM",
            XcFuncId::HYB_MGGA_XC_EDMGGAH => "HYB_MGGA_XC_EDMGGAH",
            XcFuncId::MGGA_X_MBRXC_BG => "MGGA_X_MBRXC_BG",
            XcFuncId::MGGA_X_MBRXH_BG => "MGGA_X_MBRXH_BG",
            XcFuncId::MGGA_X_HLTA => "MGGA_X_HLTA",
            XcFuncId::MGGA_C_HLTAPW => "MGGA_C_HLTAPW",
            XcFuncId::MGGA_X_SCANL => "MGGA_X_SCANL",
            XcFuncId::MGGA_X_REVSCANL => "MGGA_X_REVSCANL",
            XcFuncId::MGGA_C_SCANL => "MGGA_C_SCANL",
            XcFuncId::MGGA_C_SCANL_RVV10 => "MGGA_C_SCANL_RVV10",
            XcFuncId::MGGA_C_SCANL_VV10 => "MGGA_C_SCANL_VV10",
            XcFuncId::HYB_MGGA_X_JS18 => "HYB_MGGA_X_JS18",
            XcFuncId::HYB_MGGA_X_PJS18 => "HYB_MGGA_X_PJS18",
            XcFuncId::MGGA_X_TASK => "MGGA_X_TASK",
            XcFuncId::MGGA_X_MGGAC => "MGGA_X_MGGAC",
            XcFuncId::GGA_C_MGGAC => "GGA_C_MGGAC",
            XcFuncId::MGGA_X_MBR => "MGGA_X_MBR",
            XcFuncId::MGGA_X_R2SCANL => "MGGA_X_R2SCANL",
            XcFuncId::MGGA_C_R2SCANL => "MGGA_C_R2SCANL",
            XcFuncId::HYB_MGGA_XC_LC_TMLYP => "HYB_MGGA_XC_LC_TMLYP",
            XcFuncId::MGGA_X_MTASK => "MGGA_X_MTASK",
            XcFuncId::GGA_X_Q1D => "GGA_X_Q1D",
            XcFuncId::MGGA_X_KTBM_0 => "MGGA_X_KTBM_0",
            XcFuncId::MGGA_X_KTBM_1 => "MGGA_X_KTBM_1",
            XcFuncId::MGGA_X_KTBM_2 => "MGGA_X_KTBM_2",
            XcFuncId::MGGA_X_KTBM_3 => "MGGA_X_KTBM_3",
            XcFuncId::MGGA_X_KTBM_4 => "MGGA_X_KTBM_4",
            XcFuncId::MGGA_X_KTBM_5 => "MGGA_X_KTBM_5",
            XcFuncId::MGGA_X_KTBM_6 => "MGGA_X_KTBM_6",
            XcFuncId::MGGA_X_KTBM_7 => "MGGA_X_KTBM_7",
            XcFuncId::MGGA_X_KTBM_8 => "MGGA_X_KTBM_8",
            XcFuncId::MGGA_X_KTBM_9 => "MGGA_X_KTBM_9",
            XcFuncId::MGGA_X_KTBM_10 => "MGGA_X_KTBM_10",
            XcFuncId::MGGA_X_KTBM_11 => "MGGA_X_KTBM_11",
            XcFuncId::MGGA_X_KTBM_12 => "MGGA_X_KTBM_12",
            XcFuncId::MGGA_X_KTBM_13 => "MGGA_X_KTBM_13",
            XcFuncId::MGGA_X_KTBM_14 => "MGGA_X_KTBM_14",
            XcFuncId::MGGA_X_KTBM_15 => "MGGA_X_KTBM_15",
            XcFuncId::MGGA_X_KTBM_16 => "MGGA_X_KTBM_16",
            XcFuncId::MGGA_X_KTBM_17 => "MGGA_X_KTBM_17",
            XcFuncId::MGGA_X_KTBM_18 => "MGGA_X_KTBM_18",
            XcFuncId::MGGA_X_KTBM_19 => "MGGA_X_KTBM_19",
            XcFuncId::MGGA_X_KTBM_20 => "MGGA_X_KTBM_20",
            XcFuncId::MGGA_X_KTBM_21 => "MGGA_X_KTBM_21",
            XcFuncId::MGGA_X_KTBM_22 => "MGGA_X_KTBM_22",
            XcFuncId::MGGA_X_KTBM_23 => "MGGA_X_KTBM_23",
            XcFuncId::MGGA_X_KTBM_24 => "MGGA_X_KTBM_24",
            XcFuncId::MGGA_X_KTBM_GAP => "MGGA_X_KTBM_GAP",
            XcFuncId::MGGA_X_MSPBEL => "MGGA_X_MSPBEL",
            XcFuncId::MGGA_X_RMSPBEL => "MGGA_X_RMSPBEL",
            XcFuncId::MGGA_X_MSRPBEL => "MGGA_X_MSRPBEL",
            XcFuncId::MGGA_X_RMSRPBEL => "MGGA_X_RMSRPBEL",
            XcFuncId::MGGA_X_MSB86BL => "MGGA_X_MSB86BL",
            XcFuncId::MGGA_X_RMSB86BL => "MGGA_X_RMSB86BL",
            XcFuncId::HYB_MGGA_X_PI_M06_2X_DL => "HYB_MGGA_X_PI_M06_2X_DL",
            XcFuncId::MGGA_C_PI_M06_2X_DL => "MGGA_C_PI_M06_2X_DL",
            XcFuncId::HYB_MGGA_X_PI_M06_2X => "HYB_MGGA_X_PI_M06_2X",
            XcFuncId::MGGA_C_PI_M06_2X => "MGGA_C_PI_M06_2X",
        }
    }

    /// Get the description of this functional.
    pub fn description(self) -> &'static str {
        match self {
            XcFuncId::LDA_X => "Slater exchange",
            XcFuncId::LDA_C_WIGNER => "Wigner",
            XcFuncId::LDA_C_RPA => "Random Phase Approximation (RPA)",
            XcFuncId::LDA_C_HL => "Hedin & Lundqvist",
            XcFuncId::LDA_C_GL => "Gunnarsson & Lundqvist",
            XcFuncId::LDA_C_XALPHA => "Slater's Xalpha",
            XcFuncId::LDA_C_VWN => "Vosko, Wilk & Nusair (VWN5)",
            XcFuncId::LDA_C_VWN_RPA => "Vosko, Wilk & Nusair (VWN5_RPA)",
            XcFuncId::LDA_C_PZ => "Perdew & Zunger",
            XcFuncId::LDA_C_PZ_MOD => "Perdew & Zunger (Modified)",
            XcFuncId::LDA_C_OB_PZ => "Ortiz & Ballone (PZ parametrization)",
            XcFuncId::LDA_C_PW => "Perdew & Wang",
            XcFuncId::LDA_C_PW_MOD => "Perdew & Wang (modified)",
            XcFuncId::LDA_C_OB_PW => "Ortiz & Ballone (PW parametrization)",
            XcFuncId::LDA_C_2D_AMGB => "AMGB (for 2D systems)",
            XcFuncId::LDA_C_2D_PRM => "PRM (for 2D systems)",
            XcFuncId::LDA_C_VBH => "von Barth & Hedin",
            XcFuncId::LDA_C_1D_CSS => "Casula, Sorella & Senatore",
            XcFuncId::LDA_X_2D => "Slater exchange",
            XcFuncId::LDA_XC_TETER93 => "Teter 93",
            XcFuncId::LDA_X_1D_SOFT => "Exchange in 1D for an soft-Coulomb interaction",
            XcFuncId::LDA_C_ML1 => "Modified LSD (version 1) of Proynov and Salahub",
            XcFuncId::LDA_C_ML2 => "Modified LSD (version 2) of Proynov and Salahub",
            XcFuncId::LDA_C_GOMBAS => "Gombas",
            XcFuncId::LDA_C_PW_RPA => "Perdew & Wang (fit to the RPA energy)",
            XcFuncId::LDA_C_1D_LOOS => "P-F Loos correlation LDA",
            XcFuncId::LDA_C_RC04 => "Ragot-Cortona",
            XcFuncId::LDA_C_VWN_1 => "Vosko, Wilk & Nusair (VWN1)",
            XcFuncId::LDA_C_VWN_2 => "Vosko, Wilk & Nusair (VWN2)",
            XcFuncId::LDA_C_VWN_3 => "Vosko, Wilk & Nusair (VWN3)",
            XcFuncId::LDA_C_VWN_4 => "Vosko, Wilk & Nusair (VWN4)",
            XcFuncId::GGA_X_GAM => "Minnesota GAM exhange functional",
            XcFuncId::GGA_C_GAM => "Minnesota GAM correlation functional",
            XcFuncId::GGA_X_HCTH_A => "HCTH-A",
            XcFuncId::GGA_X_EV93 => "Engel and Vosko",
            XcFuncId::HYB_MGGA_X_DLDF => "Dispersionless Density Functional",
            XcFuncId::MGGA_C_DLDF => "Dispersionless Density Functional",
            XcFuncId::GGA_X_BCGP => "Burke, Cancio, Gould, and Pittalis",
            XcFuncId::GGA_C_ACGGA => "acGGA, asymptotically corrected GGA correlation",
            XcFuncId::GGA_X_LAMBDA_OC2_N => "lambda_OC2(N) version of PBE",
            XcFuncId::GGA_X_B86_R => "Revised Becke 86 with modified gradient correction",
            XcFuncId::MGGA_XC_ZLP => "Zhao, Levy & Parr, Eq. (21)",
            XcFuncId::LDA_XC_ZLP => "Zhao, Levy & Parr, Eq. (20)",
            XcFuncId::GGA_X_LAMBDA_CH_N => "lambda_CH(N) version of PBE",
            XcFuncId::GGA_X_LAMBDA_LO_N => "lambda_LO(N) version of PBE",
            XcFuncId::GGA_X_HJS_B88_V2 => "HJS screened exchange B88 corrected version",
            XcFuncId::GGA_C_Q2D => "Chiodo et al",
            XcFuncId::GGA_X_Q2D => "Chiodo et al",
            XcFuncId::GGA_X_PBE_MOL => "Reparametrized PBE by del Campo, Gazquez, Trickey & Vela",
            XcFuncId::LDA_K_TF => "Thomas-Fermi kinetic energy",
            XcFuncId::LDA_K_LP => "Lee and Parr Gaussian ansatz for the kinetic energy",
            XcFuncId::GGA_K_TFVW => "Thomas-Fermi plus von Weiszaecker correction",
            XcFuncId::GGA_K_REVAPBEINT => "interpolated version of revAPBE",
            XcFuncId::GGA_K_APBEINT => "interpolated version of APBE",
            XcFuncId::GGA_K_REVAPBE => "revised APBE",
            XcFuncId::GGA_X_AK13 => "Armiento & Kuemmel 2013",
            XcFuncId::GGA_K_MEYER => "Meyer,  Wang, and Young",
            XcFuncId::GGA_X_LV_RPW86 => "Berland and Hyldgaard",
            XcFuncId::GGA_X_PBE_TCA => "PBE revised by Tognetti et al",
            XcFuncId::GGA_X_PBEINT => "PBE for hybrid interfaces",
            XcFuncId::GGA_C_ZPBEINT => "spin-dependent gradient correction to PBEint",
            XcFuncId::GGA_C_PBEINT => "PBE for hybrid interfaces",
            XcFuncId::GGA_C_ZPBESOL => "spin-dependent gradient correction to PBEsol",
            XcFuncId::MGGA_XC_OTPSS_D => "oTPSS-D functional of Goerigk and Grimme",
            XcFuncId::GGA_XC_OPBE_D => "oPBE-D functional of Goerigk and Grimme",
            XcFuncId::GGA_XC_OPWLYP_D => "oPWLYP-D functional of Goerigk and Grimme",
            XcFuncId::GGA_XC_OBLYP_D => "oBLYP-D functional of Goerigk and Grimme",
            XcFuncId::GGA_X_VMT84_GE => "VMT{8,4} with constraint satisfaction with mu = mu_GE",
            XcFuncId::GGA_X_VMT84_PBE => "VMT{8,4} with constraint satisfaction with mu = mu_PBE",
            XcFuncId::GGA_X_VMT_GE => "Vela, Medel, and Trickey with mu = mu_GE",
            XcFuncId::GGA_X_VMT_PBE => "Vela, Medel, and Trickey with mu = mu_PBE",
            XcFuncId::MGGA_C_CS => "Colle and Salvetti",
            XcFuncId::MGGA_C_MN12_SX => "Minnesota MN12-SX correlation functional",
            XcFuncId::MGGA_C_MN12_L => "Minnesota MN12-L correlation functional",
            XcFuncId::MGGA_C_M11_L => "Minnesota M11-L correlation functional",
            XcFuncId::MGGA_C_M11 => "Minnesota M11 correlation functional",
            XcFuncId::MGGA_C_M08_SO => "Minnesota M08-SO correlation functional",
            XcFuncId::MGGA_C_M08_HX => "Minnesota M08 correlation functional",
            XcFuncId::GGA_C_N12_SX => "Minnesota N12-SX correlation functional",
            XcFuncId::GGA_C_N12 => "Minnesota N12 correlation functional",
            XcFuncId::HYB_GGA_X_N12_SX => "Minnesota N12-SX exchange functional",
            XcFuncId::GGA_X_N12 => "Minnesota N12 exchange functional",
            XcFuncId::GGA_C_REGTPSS => "regularized TPSS correlation",
            XcFuncId::GGA_C_OP_XALPHA => "one-parameter progressive functional (Xalpha version)",
            XcFuncId::GGA_C_OP_G96 => "one-parameter progressive functional (G96 version)",
            XcFuncId::GGA_C_OP_PBE => "one-parameter progressive functional (PBE version)",
            XcFuncId::GGA_C_OP_B88 => "one-parameter progressive functional (B88 version)",
            XcFuncId::GGA_C_FT97 => "Filatov & Thiel correlation",
            XcFuncId::GGA_C_SPBE => "PBE correlation to be used with the SSB exchange",
            XcFuncId::GGA_X_SSB_SW => "Swart, Sola and Bickelhaupt correction to PBE",
            XcFuncId::GGA_X_SSB => "Swart, Sola and Bickelhaupt",
            XcFuncId::GGA_X_SSB_D => "Swart, Sola and Bickelhaupt dispersion",
            XcFuncId::GGA_XC_HCTH_407P => "HCTH/407+",
            XcFuncId::GGA_XC_HCTH_P76 => "HCTH p=7/6",
            XcFuncId::GGA_XC_HCTH_P14 => "HCTH p=1/4",
            XcFuncId::GGA_XC_B97_GGA1 => "Becke 97 GGA-1",
            XcFuncId::GGA_C_HCTH_A => "HCTH-A",
            XcFuncId::GGA_X_BPCCAC => "BPCCAC (GRAC for the energy)",
            XcFuncId::GGA_C_REVTCA => "Tognetti, Cortona, Adamo (revised)",
            XcFuncId::GGA_C_TCA => "Tognetti, Cortona, Adamo",
            XcFuncId::GGA_X_PBE => "Perdew, Burke & Ernzerhof",
            XcFuncId::GGA_X_PBE_R => "Revised PBE from Zhang & Yang",
            XcFuncId::GGA_X_B86 => "Becke 86",
            XcFuncId::HYB_LDA_XC_B93 => "Becke's original half-and-half functional: 50% HF and 50% LDA xc",
            XcFuncId::GGA_X_B86_MGC => "Becke 86 with modified gradient correction",
            XcFuncId::GGA_X_B88 => "Becke 88",
            XcFuncId::GGA_X_G96 => "Gill 96",
            XcFuncId::GGA_X_PW86 => "Perdew & Wang 86",
            XcFuncId::GGA_X_PW91 => "Perdew & Wang 91",
            XcFuncId::GGA_X_OPTX => "Handy & Cohen OPTX 01",
            XcFuncId::GGA_X_DK87_R1 => "dePristo & Kress 87 version R1",
            XcFuncId::GGA_X_DK87_R2 => "dePristo & Kress 87 version R2",
            XcFuncId::GGA_X_LG93 => "Lacks & Gordon 93",
            XcFuncId::GGA_X_FT97_A => "Filatov & Thiel 97 (version A)",
            XcFuncId::GGA_X_FT97_B => "Filatov & Thiel 97 (version B)",
            XcFuncId::GGA_X_PBE_SOL => "Perdew, Burke & Ernzerhof SOL",
            XcFuncId::GGA_X_RPBE => "Hammer, Hansen, and Norskov",
            XcFuncId::GGA_X_WC => "Wu & Cohen",
            XcFuncId::GGA_X_MPW91 => "mPW91 of Adamo & Barone",
            XcFuncId::GGA_X_AM05 => "Armiento & Mattsson 05",
            XcFuncId::GGA_X_PBEA => "Madsen 07",
            XcFuncId::GGA_X_MPBE => "Adamo & Barone modification to PBE",
            XcFuncId::GGA_X_XPBE => "Extended PBE by Xu & Goddard III",
            XcFuncId::GGA_X_2D_B86_MGC => "Becke 86 with modified gradient correction for 2D",
            XcFuncId::GGA_X_BAYESIAN => "Bayesian best fit for the enhancement factor",
            XcFuncId::GGA_X_PBE_JSJR => "Reparametrized PBE by Pedroza, Silva & Capelle",
            XcFuncId::GGA_X_2D_B88 => "Becke 88 in 2D",
            XcFuncId::GGA_X_2D_B86 => "Becke 86 in 2D",
            XcFuncId::GGA_X_2D_PBE => "Perdew, Burke & Ernzerhof in 2D",
            XcFuncId::GGA_C_PBE => "Perdew, Burke & Ernzerhof",
            XcFuncId::GGA_C_LYP => "Lee, Yang & Parr",
            XcFuncId::GGA_C_P86 => "Perdew 86",
            XcFuncId::GGA_C_PBE_SOL => "Perdew, Burke & Ernzerhof SOL",
            XcFuncId::GGA_C_PW91 => "Perdew & Wang 91",
            XcFuncId::GGA_C_AM05 => "Armiento & Mattsson 05",
            XcFuncId::GGA_C_XPBE => "Extended PBE by Xu & Goddard III",
            XcFuncId::GGA_C_LM => "Langreth & Mehl",
            XcFuncId::GGA_C_PBE_JRGX => "Reparametrized PBE by Pedroza, Silva & Capelle",
            XcFuncId::GGA_X_OPTB88_VDW => "opt-Becke 88 for vdW",
            XcFuncId::GGA_X_PBEK1_VDW => "Reparametrized PBE for vdW",
            XcFuncId::GGA_X_OPTPBE_VDW => "Reparametrized PBE for vdW",
            XcFuncId::GGA_X_RGE2 => "Regularized PBE",
            XcFuncId::GGA_C_RGE2 => "Regularized PBE",
            XcFuncId::GGA_X_RPW86 => "Refitted Perdew & Wang 86",
            XcFuncId::GGA_X_KT1 => "Exchange part of Keal and Tozer version 1",
            XcFuncId::GGA_XC_KT2 => "Keal and Tozer, version 2",
            XcFuncId::GGA_C_WL => "Wilson & Levy",
            XcFuncId::GGA_C_WI => "Wilson & Ivanov",
            XcFuncId::GGA_X_MB88 => "Modified Becke 88 for proton transfer",
            XcFuncId::GGA_X_SOGGA => "Second-order generalized gradient approximation",
            XcFuncId::GGA_X_SOGGA11 => "Second-order generalized gradient approximation 2011",
            XcFuncId::GGA_C_SOGGA11 => "Second-order generalized gradient approximation 2011",
            XcFuncId::GGA_C_WI0 => "Wilson & Ivanov initial version",
            XcFuncId::GGA_XC_TH1 => "Tozer and Handy v. 1",
            XcFuncId::GGA_XC_TH2 => "Tozer and Handy v. 2",
            XcFuncId::GGA_XC_TH3 => "Tozer and Handy v. 3",
            XcFuncId::GGA_XC_TH4 => "Tozer and Handy v. 4",
            XcFuncId::GGA_X_C09X => "C09x to be used with the VdW of Rutgers-Chalmers",
            XcFuncId::GGA_C_SOGGA11_X => "To be used with HYB_GGA_X_SOGGA11_X",
            XcFuncId::GGA_X_LB => "van Leeuwen & Baerends",
            XcFuncId::GGA_XC_HCTH_93 => "HCTH/93",
            XcFuncId::GGA_XC_HCTH_120 => "HCTH/120",
            XcFuncId::GGA_XC_HCTH_147 => "HCTH/147",
            XcFuncId::GGA_XC_HCTH_407 => "HCTH/407",
            XcFuncId::GGA_XC_EDF1 => "EDF1",
            XcFuncId::GGA_XC_XLYP => "XLYP",
            XcFuncId::GGA_XC_KT1 => "Keal and Tozer, version 1",
            XcFuncId::GGA_X_LSPBE => "lsPBE, a PW91-like modification of PBE exchange",
            XcFuncId::GGA_X_LSRPBE => "lsRPBE, a PW91-like modification of RPBE",
            XcFuncId::GGA_XC_B97_D => "Becke 97-D",
            XcFuncId::GGA_X_OPTB86B_VDW => "Becke 86 reoptimized for use with vdW functional of Dion et al",
            XcFuncId::MGGA_C_REVM11 => "Revised Minnesota M11 correlation functional",
            XcFuncId::GGA_XC_PBE1W => "PBE1W",
            XcFuncId::GGA_XC_MPWLYP1W => "mPWLYP1w",
            XcFuncId::GGA_XC_PBELYP1W => "PBELYP1W",
            XcFuncId::GGA_C_ACGGAP => "acGGA+, asymptotically corrected GGA correlation+",
            XcFuncId::HYB_LDA_XC_LDA0 => "LDA hybrid exchange (LDA0)",
            XcFuncId::HYB_LDA_XC_CAM_LDA0 => "CAM version of LDA0",
            XcFuncId::GGA_X_B88_6311G => "Becke 88 reoptimized with the 6-311G** basis set",
            XcFuncId::GGA_X_NCAP => "Nearly correct asymptotic potential",
            XcFuncId::GGA_XC_NCAP => "NCAP exchange + P86 correlation",
            XcFuncId::GGA_X_LBM => "van Leeuwen & Baerends modified",
            XcFuncId::GGA_X_OL2 => "Exchange form based on Ou-Yang and Levy v.2",
            XcFuncId::GGA_X_APBE => "mu fixed from the semiclassical neutral atom",
            XcFuncId::GGA_K_APBE => "mu fixed from the semiclassical neutral atom",
            XcFuncId::GGA_C_APBE => "mu fixed from the semiclassical neutral atom",
            XcFuncId::GGA_K_TW1 => "Tran and Wesolowski set 1 (Table II)",
            XcFuncId::GGA_K_TW2 => "Tran and Wesolowski set 2 (Table II)",
            XcFuncId::GGA_K_TW3 => "Tran and Wesolowski set 3 (Table II)",
            XcFuncId::GGA_K_TW4 => "Tran and Wesolowski set 4 (Table II)",
            XcFuncId::GGA_X_HTBS => "Haas, Tran, Blaha, and Schwarz",
            XcFuncId::GGA_X_AIRY => "Constantin et al based on the Airy gas",
            XcFuncId::GGA_X_LAG => "Local Airy Gas",
            XcFuncId::GGA_XC_MOHLYP => "Functional for organometallic chemistry",
            XcFuncId::GGA_XC_MOHLYP2 => "Functional for barrier heights",
            XcFuncId::LDA_XC_TH_FL => "Tozer and Handy v. FL",
            XcFuncId::GGA_XC_TH_FC => "Tozer and Handy v. FC",
            XcFuncId::GGA_XC_TH_FCFO => "Tozer and Handy v. FCFO",
            XcFuncId::GGA_XC_TH_FCO => "Tozer and Handy v. FCO",
            XcFuncId::GGA_C_OPTC => "Optimized correlation functional of Cohen and Handy",
            XcFuncId::MGGA_X_LTA => "Local tau approximation",
            XcFuncId::MGGA_X_TPSS => "Tao, Perdew, Staroverov & Scuseria",
            XcFuncId::MGGA_X_M06_L => "Minnesota M06-L exchange functional",
            XcFuncId::MGGA_X_GVT4 => "GVT4 (X part of VSXC)",
            XcFuncId::MGGA_X_TAU_HCTH => "tau-HCTH from Boese and Handy",
            XcFuncId::MGGA_X_BR89 => "Becke-Roussel 89, gamma = 0.8",
            XcFuncId::MGGA_X_BJ06 => "Becke & Johnson 06",
            XcFuncId::MGGA_X_TB09 => "Tran & Blaha 09",
            XcFuncId::MGGA_X_RPP09 => "Rasanen, Pittalis & Proetto 09",
            XcFuncId::MGGA_X_2D_PRHG07 => "Pittalis-Rasanen-Helbig-Gross 2007",
            XcFuncId::MGGA_X_2D_PRHG07_PRP10 => "PRHG07 with Pittalis-Rasanen-Proetto 2010 correction",
            XcFuncId::MGGA_X_REVTPSS => "revised Tao, Perdew, Staroverov & Scuseria",
            XcFuncId::MGGA_X_PKZB => "Perdew, Kurth, Zupan, and Blaha",
            XcFuncId::MGGA_X_BR89_1 => "Becke-Roussel 89, gamma = 1.0",
            XcFuncId::GGA_X_ECMV92 => "Engel, Chevary, Macdonald and Vosko",
            XcFuncId::GGA_C_PBE_VWN => "Perdew, Burke & Ernzerhof based on VWN correlation",
            XcFuncId::GGA_C_P86_FT => "Perdew 86 with more accurate value for ftilde",
            XcFuncId::GGA_K_RATIONAL_P => "RATIONAL$^{p}$ by Lehtomaki and Lopez-Acevedo (by default $p=3/2$, $C_{2}=0.7687$)",
            XcFuncId::GGA_K_PG1 => "PG1 (Pauli-Gaussian) functional by Constantin, Fabiano, and Della Sala",
            XcFuncId::MGGA_K_PGSL025 => "PGSL025 (Pauli-Gaussian) functional by Constantin, Fabiano, and Della Sala",
            XcFuncId::MGGA_X_MS0 => "MS exchange of Sun, Xiao, and Ruzsinszky",
            XcFuncId::MGGA_X_MS1 => "MS1 exchange of Sun, et al",
            XcFuncId::MGGA_X_MS2 => "MS2 exchange of Sun, et al",
            XcFuncId::HYB_MGGA_X_MS2H => "MS2 hybrid exchange of Sun, et al",
            XcFuncId::MGGA_X_TH => "Tsuneda and Hirao",
            XcFuncId::MGGA_X_M11_L => "Minnesota M11-L exchange functional",
            XcFuncId::MGGA_X_MN12_L => "Minnesota MN12-L exchange functional",
            XcFuncId::MGGA_X_MS2_REV => "MS2 exchange of Sun, et al with revised value for c",
            XcFuncId::MGGA_XC_CC06 => "Cancio and Chou 2006",
            XcFuncId::MGGA_X_GP86 => "Ghosh-Parr 1986 meta-GGA exchange, later reinvestigated by Manby and Knowles",
            XcFuncId::MGGA_C_TPSS => "Tao, Perdew, Staroverov & Scuseria",
            XcFuncId::MGGA_C_VSXC => "VSXC (correlation part)",
            XcFuncId::MGGA_C_M06_L => "Minnesota M06-L correlation functional",
            XcFuncId::MGGA_C_M06_HF => "Minnesota M06-HF correlation functional",
            XcFuncId::MGGA_C_M06 => "Minnesota M06 correlation functional",
            XcFuncId::MGGA_C_M06_2X => "Minnesota M06-2X correlation functional",
            XcFuncId::MGGA_C_M05 => "Minnesota M05 correlation functional",
            XcFuncId::MGGA_C_M05_2X => "Minnesota M05-2X correlation functional",
            XcFuncId::MGGA_C_PKZB => "Perdew, Kurth, Zupan, and Blaha",
            XcFuncId::MGGA_C_BC95 => "Becke correlation 95",
            XcFuncId::MGGA_C_REVTPSS => "revised TPSS correlation",
            XcFuncId::MGGA_XC_TPSSLYP1W => "TPSSLYP1W",
            XcFuncId::MGGA_X_MK00B => "Exchange for accurate virtual orbital energies (v. B)",
            XcFuncId::MGGA_X_BLOC => "functional with balanced localization",
            XcFuncId::MGGA_X_MODTPSS => "Modified Tao, Perdew, Staroverov & Scuseria",
            XcFuncId::GGA_C_PBELOC => "Semilocal dynamical correlation",
            XcFuncId::MGGA_C_TPSSLOC => "Semilocal dynamical correlation",
            XcFuncId::HYB_MGGA_X_MN12_SX => "Minnesota MN12-SX hybrid exchange functional",
            XcFuncId::MGGA_X_MBEEF => "mBEEF exchange",
            XcFuncId::MGGA_X_MBEEFVDW => "mBEEF-vdW exchange",
            XcFuncId::MGGA_C_TM => "Tao and Mo 2016 correlation",
            XcFuncId::GGA_C_P86VWN => "Perdew 86 based on VWN5 correlation",
            XcFuncId::GGA_C_P86VWN_FT => "Perdew 86 based on VWN5 correlation, with more accurate value for ftilde",
            XcFuncId::MGGA_XC_B97M_V => "B97M-V exchange-correlation functional",
            XcFuncId::GGA_XC_VV10 => "Vydrov and Van Voorhis",
            XcFuncId::MGGA_X_JK => "Jemmer-Knowles meta-GGA exchange",
            XcFuncId::MGGA_X_MVS => "MVS exchange of Sun, Perdew, and Ruzsinszky",
            XcFuncId::GGA_C_PBEFE => "PBE for formation energies",
            XcFuncId::LDA_XC_KSDT => "Karasiev, Sjostrom, Dufty & Trickey",
            XcFuncId::MGGA_X_MN15_L => "Minnesota MN15-L exchange functional",
            XcFuncId::MGGA_C_MN15_L => "Minnesota MN15-L correlation functional",
            XcFuncId::GGA_C_OP_PW91 => "one-parameter progressive functional (PW91 version)",
            XcFuncId::MGGA_X_SCAN => "SCAN exchange of Sun, Ruzsinszky, and Perdew",
            XcFuncId::HYB_MGGA_X_SCAN0 => "SCAN hybrid exchange (SCAN0)",
            XcFuncId::GGA_X_PBEFE => "PBE for formation energies",
            XcFuncId::HYB_GGA_XC_B97_1P => "version of B97 by Cohen and Handy",
            XcFuncId::MGGA_C_SCAN => "SCAN correlation of Sun, Ruzsinszky, and Perdew",
            XcFuncId::HYB_MGGA_X_MN15 => "Minnesota MN15 hybrid exchange functional",
            XcFuncId::MGGA_C_MN15 => "Minnesota MN15 correlation functional",
            XcFuncId::GGA_X_CAP => "Correct Asymptotic Potential",
            XcFuncId::GGA_X_EB88 => "Non-empirical (excogitated) B88 functional of Becke and Elliott",
            XcFuncId::GGA_C_PBE_MOL => "Reparametrized PBE by del Campo, Gazquez, Trickey & Vela",
            XcFuncId::HYB_GGA_XC_PBE_MOL0 => "PBEmol0",
            XcFuncId::HYB_GGA_XC_PBE_SOL0 => "PBEsol0",
            XcFuncId::HYB_GGA_XC_PBEB0 => "PBEbeta0",
            XcFuncId::HYB_GGA_XC_PBE_MOLB0 => "PBEmolbeta0",
            XcFuncId::GGA_K_ABSP3 => "gamma-TFvW form by Acharya et al [$g = 1 - 1.513/N^{0.35}]$",
            XcFuncId::GGA_K_ABSP4 => "gamma-TFvW form by Acharya et al [$g = l = 1/(1 + 1.332/N^{1/3})$]",
            XcFuncId::HYB_MGGA_X_BMK => "Boese-Martin for kinetics",
            XcFuncId::GGA_C_BMK => "Boese-Martin correlation for kinetics",
            XcFuncId::GGA_C_TAU_HCTH => "correlation part of tau-hcth",
            XcFuncId::HYB_MGGA_X_TAU_HCTH => "Hybrid version of tau-HCTH",
            XcFuncId::GGA_C_HYB_TAU_HCTH => "correlation part of hyb-tau-hcth",
            XcFuncId::MGGA_X_B00 => "Becke 2000",
            XcFuncId::GGA_X_BEEFVDW => "BEEF-vdW exchange",
            XcFuncId::GGA_XC_BEEFVDW => "BEEF-vdW exchange-correlation",
            XcFuncId::LDA_C_CHACHIYO => "Chachiyo simple 2 parameter correlation",
            XcFuncId::MGGA_XC_HLE17 => "high local exchange 2017",
            XcFuncId::LDA_C_LP96 => "Liu-Parr correlation",
            XcFuncId::HYB_GGA_XC_PBE50 => "PBE50",
            XcFuncId::GGA_X_PBETRANS => "Gradient-regulated connection-based correction for the PBE exchange",
            XcFuncId::MGGA_C_SCAN_RVV10 => "SCAN + rVV10 correlation",
            XcFuncId::MGGA_X_REVM06_L => "Minnesota revM06-L exchange functional",
            XcFuncId::MGGA_C_REVM06_L => "Minnesota revM06-L correlation functional",
            XcFuncId::HYB_MGGA_X_M08_HX => "Minnesota M08-HX hybrid exchange functional",
            XcFuncId::HYB_MGGA_X_M08_SO => "Minnesota M08-SO hybrid exchange functional",
            XcFuncId::HYB_MGGA_X_M11 => "Minnesota M11 hybrid exchange functional",
            XcFuncId::GGA_X_CHACHIYO => "Chachiyo exchange",
            XcFuncId::MGGA_X_RTPSS => "TPSS for surface adsorption",
            XcFuncId::MGGA_X_MS2B => "MS2beta exchange of Furness and Sun",
            XcFuncId::MGGA_X_MS2BS => "MS2beta* exchange of Furness and Sun",
            XcFuncId::MGGA_X_MVSB => "MVSbeta exchange by Furness and Sun",
            XcFuncId::MGGA_X_MVSBS => "MVSbeta* exchange by Furness and Sun",
            XcFuncId::HYB_MGGA_X_REVM11 => "Revised Minnesota M11 hybrid exchange functional",
            XcFuncId::HYB_MGGA_X_REVM06 => "Revised Minnesota M06 hybrid exchange functional",
            XcFuncId::MGGA_C_REVM06 => "Revised Minnesota M06 correlation functional",
            XcFuncId::LDA_C_CHACHIYO_MOD => "Chachiyo simple 2 parameter correlation with modified spin scaling",
            XcFuncId::LDA_C_KARASIEV_MOD => "Karasiev reparameterization of Chachiyo",
            XcFuncId::GGA_C_CHACHIYO => "Chachiyo simple GGA correlation",
            XcFuncId::HYB_MGGA_X_M06_SX => "Minnesota M06-SX short-range hybrid exchange functional",
            XcFuncId::MGGA_C_M06_SX => "Minnesota M06-SX correlation functional",
            XcFuncId::GGA_X_REVSSB_D => "Revised Swart, Sola and Bickelhaupt dispersion",
            XcFuncId::GGA_C_CCDF => "ccDF: coupled-cluster motivated density functional",
            XcFuncId::HYB_GGA_XC_HFLYP => "HF + LYP correlation",
            XcFuncId::HYB_GGA_XC_B3P86_NWCHEM => "B3P86, NWChem version",
            XcFuncId::GGA_X_PW91_MOD => "PW91, alternate version with more digits",
            XcFuncId::LDA_C_W20 => "Xie, Wu, and Zhao interpolation ansatz without fitting parameters",
            XcFuncId::LDA_XC_CORRKSDT => "Corrected KSDT by Karasiev, Dufty and Trickey",
            XcFuncId::MGGA_X_FT98 => "Filatov and Thiel 1998 meta-GGA exchange",
            XcFuncId::GGA_X_PBE_MOD => "Perdew, Burke & Ernzerhof with less precise value for beta",
            XcFuncId::GGA_X_PBE_GAUSSIAN => "Perdew, Burke & Ernzerhof with parameter values used in Gaussian",
            XcFuncId::GGA_C_PBE_GAUSSIAN => "Perdew, Burke & Ernzerhof with parameters from Gaussian",
            XcFuncId::MGGA_C_TPSS_GAUSSIAN => "Tao, Perdew, Staroverov & Scuseria with parameters from Gaussian",
            XcFuncId::GGA_X_NCAPR => "Nearly correct asymptotic potential revised",
            XcFuncId::HYB_GGA_XC_RELPBE0 => "relPBE0 a.k.a. relPBE: PBE0 refitted for actinide compounds",
            XcFuncId::MGGA_X_EEL => "Exact exchange-like exchange of Aschebrock et al",
            XcFuncId::GGA_XC_B97_3C => "Becke 97-3c by Grimme et. al.",
            XcFuncId::LDA_C_EPC17 => "epc17(-1): electron-proton correlation 2017",
            XcFuncId::LDA_C_EPC17_2 => "epc17-2: electron-proton correlation 2017 for proton affinities",
            XcFuncId::LDA_C_EPC18_1 => "epc18-1: electron-proton correlation 2018",
            XcFuncId::LDA_C_EPC18_2 => "epc18-2: electron-proton correlation 2018 for proton affinities",
            XcFuncId::GGA_XC_DLB97 => "dispersionless-optimized B97",
            XcFuncId::MGGA_X_MSCAN => "Modified SCAN (mSCAN) exchange of Desmarais, Erba, Vignale, and Pittalis",
            XcFuncId::MGGA_C_MSCAN => "Modified SCAN (mSCAN) correlation of Desmarais, Erba, Vignale, and Pittalis",
            XcFuncId::GGA_X_T_PBE1 => "PBE reparametrization (version 1) for band gaps",
            XcFuncId::GGA_X_T_PBE2 => "PBE reparametrization (version 2) for band gaps",
            XcFuncId::LDA_X_T_SLOC => "SLOC reparametrization for band gaps",
            XcFuncId::GGA_X_BKL1 => "Exchange part of type-I band gap functional by Bhattacharjee, Koshi and Lee",
            XcFuncId::GGA_X_BKL2 => "Exchange part of type-II band gap functional by Bhattacharjee, Koshi and Lee",
            XcFuncId::HYB_MGGA_X_CF22D => "Minnesota CF22D hybrid exchange functional",
            XcFuncId::MGGA_C_CF22D => "Minnesota CF22D correlation functional",
            XcFuncId::MGGA_X_LAK => "Lebeda-Aschebrock-Kummel meta-GGA exchange",
            XcFuncId::GGA_C_BKL1 => "Correlation part of type-I band gap functional by Bhattacharjee, Koshi and Lee",
            XcFuncId::GGA_C_BKL2 => "Correlation part of type-II band gap functional by Bhattacharjee, Koshi and Lee",
            XcFuncId::MGGA_C_LAK => "Lebeda-Aschebrock-Kummel meta-GGA correlation",
            XcFuncId::GGA_X_DF3_OPT1 => "Becke 88 reoptimized by Chakraborty et al for use with vdW functional",
            XcFuncId::GGA_X_DF3_OPT2 => "Becke 86 reoptimized by Chakraborty et al for use with vdW functional",
            XcFuncId::HYB_GGA_XC_CQTP25 => "CAM-B3LYP retuned for core electron ionization energies",
            XcFuncId::HYB_GGA_XC_OPB3LYP => "opB3LYP: B3LYP reoptimized in 6-311++G(2d,2p) basis set",
            XcFuncId::MGGA_C_CC => "Self-interaction corrected correlation functional by Schmidt et al",
            XcFuncId::MGGA_C_CCALDA => "Iso-orbital corrected LDA correlation by Lebeda et al",
            XcFuncId::HYB_MGGA_XC_BR3P86 => "BR3P86 hybrid meta-GGA from Neumann and Handy",
            XcFuncId::HYB_GGA_XC_CASE21 => "CASE21: Constrained And Smoothed semi-Empirical 2021 functional",
            XcFuncId::MGGA_C_RREGTM => "Revised regTM correlation by Jana et al",
            XcFuncId::HYB_GGA_XC_PBE_2X => "PBE-2X: PBE0 with 56% exact exchange",
            XcFuncId::HYB_GGA_XC_PBE38 => "PBE38: PBE0 with 3/8 = 37.5% exact exchange",
            XcFuncId::HYB_GGA_XC_B3LYP3 => "B3LYP with VWN functional 3 instead of RPA",
            XcFuncId::HYB_GGA_XC_CAM_O3LYP => "CAM-O3LYP",
            XcFuncId::HYB_MGGA_XC_TPSS0 => "TPSS0 with 25% exact exchange",
            XcFuncId::MGGA_C_B94 => "Becke 1994 meta-GGA correlation",
            XcFuncId::HYB_MGGA_XC_B94_HYB => "Becke 1994 hybrid meta-GGA",
            XcFuncId::HYB_GGA_XC_WB97X_D3 => "wB97X-D3 range-separated functional",
            XcFuncId::HYB_GGA_XC_LC_BLYP => "LC version of BLYP",
            XcFuncId::HYB_GGA_XC_B3PW91 => "The original (ACM, B3PW91) hybrid of Becke",
            XcFuncId::HYB_GGA_XC_B3LYP => "B3LYP",
            XcFuncId::HYB_GGA_XC_B3P86 => "B3P86",
            XcFuncId::HYB_GGA_XC_O3LYP => "O3LYP",
            XcFuncId::HYB_GGA_XC_MPW1K => "mPW1K",
            XcFuncId::HYB_GGA_XC_PBEH => "PBEH (PBE0)",
            XcFuncId::HYB_GGA_XC_B97 => "Becke 97",
            XcFuncId::HYB_GGA_XC_B97_1 => "Becke 97-1",
            XcFuncId::HYB_GGA_XC_APF => "APF hybrid functional",
            XcFuncId::HYB_GGA_XC_B97_2 => "Becke 97-2",
            XcFuncId::HYB_GGA_XC_X3LYP => "X3LYP",
            XcFuncId::HYB_GGA_XC_B1WC => "B1WC",
            XcFuncId::HYB_GGA_XC_B97_K => "Boese-Martin for Kinetics",
            XcFuncId::HYB_GGA_XC_B97_3 => "Becke 97-3",
            XcFuncId::HYB_GGA_XC_MPW3PW => "MPW3PW of Adamo & Barone",
            XcFuncId::HYB_GGA_XC_B1LYP => "B1LYP",
            XcFuncId::HYB_GGA_XC_B1PW91 => "B1PW91",
            XcFuncId::HYB_GGA_XC_MPW1PW => "mPW1PW",
            XcFuncId::HYB_GGA_XC_MPW3LYP => "MPW3LYP",
            XcFuncId::HYB_GGA_XC_SB98_1A => "SB98 (1a)",
            XcFuncId::HYB_GGA_XC_SB98_1B => "SB98 (1b)",
            XcFuncId::HYB_GGA_XC_SB98_1C => "SB98 (1c)",
            XcFuncId::HYB_GGA_XC_SB98_2A => "SB98 (2a)",
            XcFuncId::HYB_GGA_XC_SB98_2B => "SB98 (2b)",
            XcFuncId::HYB_GGA_XC_SB98_2C => "SB98 (2c)",
            XcFuncId::HYB_GGA_X_SOGGA11_X => "Hybrid based on SOGGA11 form",
            XcFuncId::HYB_GGA_XC_HSE03 => "HSE03",
            XcFuncId::HYB_GGA_XC_HSE06 => "HSE06",
            XcFuncId::HYB_GGA_XC_HJS_PBE => "HJS hybrid screened exchange PBE version",
            XcFuncId::HYB_GGA_XC_HJS_PBE_SOL => "HJS hybrid screened exchange PBE_SOL version",
            XcFuncId::HYB_GGA_XC_HJS_B88 => "HJS hybrid screened exchange B88 version",
            XcFuncId::HYB_GGA_XC_HJS_B97X => "HJS hybrid screened exchange B97x version",
            XcFuncId::HYB_GGA_XC_CAM_B3LYP => "CAM version of B3LYP",
            XcFuncId::HYB_GGA_XC_TUNED_CAM_B3LYP => "CAM version of B3LYP, tuned for excitations and properties",
            XcFuncId::HYB_GGA_XC_BHANDH => "BHandH: 50% LDA exchange and 50% HF exchange with 100% LYP correlation",
            XcFuncId::HYB_GGA_XC_BHANDHLYP => "BHandHLYP a.k.a. BHLYP: 50% B88 exchange and 50% HF exchange with 100% LYP correlation",
            XcFuncId::HYB_GGA_XC_MB3LYP_RC04 => "B3LYP with RC04 LDA",
            XcFuncId::HYB_MGGA_X_M05 => "Minnesota M05 hybrid exchange functional",
            XcFuncId::HYB_MGGA_X_M05_2X => "Minnesota M05-2X hybrid exchange functional",
            XcFuncId::HYB_MGGA_XC_B88B95 => "Mixture of B88 with BC95 (B1B95)",
            XcFuncId::HYB_MGGA_XC_B86B95 => "Mixture of B86 with BC95",
            XcFuncId::HYB_MGGA_XC_PW86B95 => "Mixture of PW86 with BC95",
            XcFuncId::HYB_MGGA_XC_BB1K => "Mixture of B88 with BC95 from Zhao and Truhlar",
            XcFuncId::HYB_MGGA_X_M06_HF => "Minnesota M06-HF hybrid exchange functional",
            XcFuncId::HYB_MGGA_XC_MPW1B95 => "Mixture of mPW91 with BC95 from Zhao and Truhlar",
            XcFuncId::HYB_MGGA_XC_MPWB1K => "Mixture of mPW91 with BC95 for kinetics",
            XcFuncId::HYB_MGGA_XC_X1B95 => "Mixture of X with BC95",
            XcFuncId::HYB_MGGA_XC_XB1K => "Mixture of X with BC95 for kinetics",
            XcFuncId::HYB_MGGA_X_M06 => "Minnesota M06 hybrid exchange functional",
            XcFuncId::HYB_MGGA_X_M06_2X => "Minnesota M06-2X hybrid exchange functional",
            XcFuncId::HYB_MGGA_XC_PW6B95 => "Mixture of PW91 with BC95 from Zhao and Truhlar",
            XcFuncId::HYB_MGGA_XC_PWB6K => "Mixture of PW91 with BC95 from Zhao and Truhlar for kinetics",
            XcFuncId::HYB_GGA_XC_MPWLYP1M => "MPW with 1 par. for metals/LYP",
            XcFuncId::HYB_GGA_XC_REVB3LYP => "Revised B3LYP",
            XcFuncId::HYB_GGA_XC_CAMY_BLYP => "CAMY version of BLYP",
            XcFuncId::HYB_GGA_XC_PBE0_13 => "PBE0-1/3",
            XcFuncId::HYB_MGGA_XC_TPSSH => "TPSSh",
            XcFuncId::HYB_MGGA_XC_REVTPSSH => "revTPSSh",
            XcFuncId::HYB_GGA_XC_B3LYPS => "B3LYP*",
            XcFuncId::HYB_GGA_XC_QTP17 => "Global hybrid for vertical ionization potentials",
            XcFuncId::HYB_GGA_XC_B3LYP_MCM1 => "B3LYP-MCM1",
            XcFuncId::HYB_GGA_XC_B3LYP_MCM2 => "B3LYP-MCM2",
            XcFuncId::HYB_GGA_XC_WB97 => "wB97 range-separated functional",
            XcFuncId::HYB_GGA_XC_WB97X => "wB97X range-separated functional",
            XcFuncId::HYB_GGA_XC_LRC_WPBEH => "Long-range corrected short-range hybrid PBE (LRC-wPBEh) by Rohrdanz, Martins and Herbert",
            XcFuncId::HYB_GGA_XC_WB97X_V => "wB97X-V range-separated functional",
            XcFuncId::HYB_GGA_XC_LCY_PBE => "LCY version of PBE",
            XcFuncId::HYB_GGA_XC_LCY_BLYP => "LCY version of BLYP",
            XcFuncId::HYB_GGA_XC_LC_VV10 => "Vydrov and Van Voorhis",
            XcFuncId::HYB_GGA_XC_CAMY_B3LYP => "CAMY version of B3LYP",
            XcFuncId::HYB_GGA_XC_WB97X_D => "wB97X-D range-separated functional",
            XcFuncId::HYB_GGA_XC_HPBEINT => "hPBEint",
            XcFuncId::HYB_GGA_XC_LRC_WPBE => "Long-range corrected PBE (LRC-wPBE) by Rohrdanz, Martins and Herbert",
            XcFuncId::HYB_MGGA_X_MVSH => "MVSh hybrid exchange functional",
            XcFuncId::HYB_GGA_XC_B3LYP5 => "B3LYP with VWN functional 5 instead of RPA",
            XcFuncId::HYB_GGA_XC_EDF2 => "EDF2",
            XcFuncId::HYB_GGA_XC_CAP0 => "Correct Asymptotic Potential hybrid",
            XcFuncId::HYB_GGA_XC_LC_WPBE => "Long-range corrected PBE (LC-wPBE) by Vydrov and Scuseria",
            XcFuncId::HYB_GGA_XC_HSE12 => "HSE12",
            XcFuncId::HYB_GGA_XC_HSE12S => "HSE12 (short-range version)",
            XcFuncId::HYB_GGA_XC_HSE_SOL => "HSEsol",
            XcFuncId::HYB_GGA_XC_CAM_QTP_01 => "CAM-B3LYP retuned using ionization potentials of water",
            XcFuncId::HYB_GGA_XC_MPW1LYP => "mPW1LYP",
            XcFuncId::HYB_GGA_XC_MPW1PBE => "mPW1PBE",
            XcFuncId::HYB_GGA_XC_KMLYP => "Kang-Musgrave hybrid",
            XcFuncId::HYB_GGA_XC_LC_WPBE_WHS => "Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria",
            XcFuncId::HYB_GGA_XC_LC_WPBEH_WHS => "Long-range corrected short-range hybrid PBE (LC-wPBE) by Weintraub, Henderson and Scuseria",
            XcFuncId::HYB_GGA_XC_LC_WPBE08_WHS => "Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria",
            XcFuncId::HYB_GGA_XC_LC_WPBESOL_WHS => "Long-range corrected PBE (LC-wPBE) by Weintraub, Henderson and Scuseria",
            XcFuncId::HYB_GGA_XC_CAM_QTP_00 => "CAM-B3LYP retuned using ionization potentials of water",
            XcFuncId::HYB_GGA_XC_CAM_QTP_02 => "CAM-B3LYP retuned using ionization potentials of water",
            XcFuncId::HYB_GGA_XC_LC_QTP => "CAM-B3LYP retuned using ionization potentials of water",
            XcFuncId::MGGA_X_RSCAN => "Regularized SCAN exchange by Bartok and Yates",
            XcFuncId::MGGA_C_RSCAN => "Regularized SCAN correlation by Bartok and Yates",
            XcFuncId::GGA_X_S12G => "Swart 2012 GGA exchange",
            XcFuncId::HYB_GGA_X_S12H => "Swart 2012 hybrid GGA exchange",
            XcFuncId::MGGA_X_R2SCAN => "Re-regularized SCAN exchange by Furness et al",
            XcFuncId::MGGA_C_R2SCAN => "Re-regularized SCAN correlation by Furness et al",
            XcFuncId::HYB_GGA_XC_BLYP35 => "BLYP35",
            XcFuncId::GGA_K_VW => "von Weiszaecker correction to Thomas-Fermi",
            XcFuncId::GGA_K_GE2 => "Second-order gradient expansion of the kinetic energy density",
            XcFuncId::GGA_K_GOLDEN => "TF-lambda-vW form by Golden (l = 13/45)",
            XcFuncId::GGA_K_YT65 => "TF-lambda-vW form by Yonei and Tomishima (l = 1/5)",
            XcFuncId::GGA_K_BALTIN => "TF-lambda-vW form by Baltin (l = 5/9)",
            XcFuncId::GGA_K_LIEB => "TF-lambda-vW form by Lieb (l = 0.185909191)",
            XcFuncId::GGA_K_ABSP1 => "gamma-TFvW form by Acharya et al [$g = 1 - 1.412/N^{1/3}$]",
            XcFuncId::GGA_K_ABSP2 => "gamma-TFvW form by Acharya et al [$g = 1 - 1.332/N^{1/3}$]",
            XcFuncId::GGA_K_GR => "gamma-TFvW form by Gazquez and Robles",
            XcFuncId::GGA_K_LUDENA => "gamma-TFvW form by Ludena",
            XcFuncId::GGA_K_GP85 => "gamma-TFvW form by Ghosh and Parr",
            XcFuncId::GGA_K_PEARSON => "Pearson 1992",
            XcFuncId::GGA_K_OL1 => "Ou-Yang and Levy v.1",
            XcFuncId::GGA_K_OL2 => "Ou-Yang and Levy v.2",
            XcFuncId::GGA_K_FR_B88 => "Fuentealba & Reyes (B88 version)",
            XcFuncId::GGA_K_FR_PW86 => "Fuentealba & Reyes (PW86 version)",
            XcFuncId::GGA_K_DK => "DePristo and Kress",
            XcFuncId::GGA_K_PERDEW => "Perdew",
            XcFuncId::GGA_K_VSK => "Vitos, Skriver, and Kollar",
            XcFuncId::GGA_K_VJKS => "Vitos, Johansson, Kollar, and Skriver",
            XcFuncId::GGA_K_ERNZERHOF => "Ernzerhof",
            XcFuncId::GGA_K_LC94 => "Lembarki & Chermette",
            XcFuncId::GGA_K_LLP => "Lee, Lee & Parr",
            XcFuncId::GGA_K_THAKKAR => "Thakkar 1992",
            XcFuncId::GGA_X_WPBEH => "short-range part of the PBE (default w=0 gives PBEh)",
            XcFuncId::GGA_X_HJS_PBE => "HJS screened exchange PBE version",
            XcFuncId::GGA_X_HJS_PBE_SOL => "HJS screened exchange PBE_SOL version",
            XcFuncId::GGA_X_HJS_B88 => "HJS screened exchange B88 version",
            XcFuncId::GGA_X_HJS_B97X => "HJS screened exchange B97x version",
            XcFuncId::GGA_X_ITYH => "Short-range recipe for B88 functional - erf",
            XcFuncId::GGA_X_SFAT => "Short-range recipe for B88 functional - Yukawa",
            XcFuncId::HYB_MGGA_XC_WB97M_V => "wB97M-V exchange-correlation functional",
            XcFuncId::LDA_X_REL => "Slater exchange with relativistic corrections",
            XcFuncId::GGA_X_SG4 => "Semiclassical GGA at fourth order",
            XcFuncId::GGA_C_SG4 => "Semiclassical GGA at fourth order",
            XcFuncId::GGA_X_GG99 => "Gilbert and Gill 1999",
            XcFuncId::LDA_XC_1D_EHWLRG_1 => "LDA constructed from slab-like systems of 1 electron",
            XcFuncId::LDA_XC_1D_EHWLRG_2 => "LDA constructed from slab-like systems of 2 electrons",
            XcFuncId::LDA_XC_1D_EHWLRG_3 => "LDA constructed from slab-like systems of 3 electrons",
            XcFuncId::GGA_X_PBEPOW => "PBE power",
            XcFuncId::MGGA_X_TM => "Tao and Mo 2016 exchange",
            XcFuncId::MGGA_X_VT84 => "meta-GGA version of VT{8,4} GGA",
            XcFuncId::MGGA_X_SA_TPSS => "TPSS with correct surface asymptotics",
            XcFuncId::MGGA_K_PC07 => "Perdew and Constantin 2007",
            XcFuncId::GGA_X_KGG99 => "Gilbert and Gill 1999 (mixed)",
            XcFuncId::GGA_XC_HLE16 => "high local exchange 2016",
            XcFuncId::LDA_X_ERF => "Short-range LDA exchange with error function kernel (erfc)",
            XcFuncId::LDA_XC_LP_A => "Lee-Parr reparametrization A",
            XcFuncId::LDA_XC_LP_B => "Lee-Parr reparametrization B",
            XcFuncId::LDA_X_RAE => "Rae self-energy corrected exchange",
            XcFuncId::LDA_K_ZLP => "Wigner including kinetic energy contribution",
            XcFuncId::LDA_C_MCWEENY => "McWeeny 76",
            XcFuncId::LDA_C_BR78 => "Brual & Rothstein 78",
            XcFuncId::GGA_C_SCAN_E0 => "GGA component of SCAN",
            XcFuncId::LDA_C_PK09 => "Proynov and Kong 2009",
            XcFuncId::GGA_C_GAPC => "GapC",
            XcFuncId::GGA_C_GAPLOC => "Gaploc",
            XcFuncId::GGA_C_ZVPBEINT => "another spin-dependent correction to PBEint",
            XcFuncId::GGA_C_ZVPBESOL => "another spin-dependent correction to PBEsol",
            XcFuncId::GGA_C_TM_LYP => "Thakkar and McCarthy reparametrization, also known as reLYP",
            XcFuncId::GGA_C_TM_PBE => "Thakkar and McCarthy reparametrization",
            XcFuncId::GGA_C_W94 => "Wilson 94 (Eq. 25)",
            XcFuncId::MGGA_C_KCIS => "Krieger, Chen, Iafrate, and Savin",
            XcFuncId::HYB_MGGA_XC_B0KCIS => "Hybrid based on KCIS",
            XcFuncId::MGGA_XC_LP90 => "Lee & Parr, Eq. (60)",
            XcFuncId::GGA_C_CS1 => "A dynamical correlation functional",
            XcFuncId::HYB_MGGA_XC_MPW1KCIS => "MPW1KCIS for barrier heights",
            XcFuncId::HYB_MGGA_XC_MPWKCIS1K => "MPWKCIS1K for barrier heights",
            XcFuncId::HYB_MGGA_XC_PBE1KCIS => "PBE1KCIS for binding energies",
            XcFuncId::HYB_MGGA_XC_TPSS1KCIS => "TPSS1KCIS for thermochemistry and kinetics",
            XcFuncId::GGA_X_B88M => "Becke 88 reoptimized to be used with tau1",
            XcFuncId::MGGA_C_B88 => "Meta-GGA correlation by Becke",
            XcFuncId::HYB_GGA_XC_B5050LYP => "B5050LYP",
            XcFuncId::LDA_C_OW_LYP => "Wigner with corresponding LYP parameters",
            XcFuncId::LDA_C_OW => "Optimized Wigner",
            XcFuncId::MGGA_X_GX => "GX functional of Loos",
            XcFuncId::MGGA_X_PBE_GX => "PBE-GX functional of Loos",
            XcFuncId::LDA_XC_GDSMFB => "Groth, Dornheim, Sjostrom, Malone, Foulkes, Bonitz",
            XcFuncId::LDA_C_GK72 => "Gordon and Kim 1972",
            XcFuncId::LDA_C_KARASIEV => "Karasiev reparameterization of Chachiyo",
            XcFuncId::LDA_K_LP96 => "Liu-Parr kinetic",
            XcFuncId::MGGA_X_REVSCAN => "revised SCAN",
            XcFuncId::MGGA_C_REVSCAN => "revised SCAN",
            XcFuncId::HYB_MGGA_X_REVSCAN0 => "revised SCAN hybrid exchange (SCAN0)",
            XcFuncId::MGGA_C_SCAN_VV10 => "SCAN + VV10 correlation",
            XcFuncId::MGGA_C_REVSCAN_VV10 => "REVSCAN + VV10 correlation",
            XcFuncId::MGGA_X_BR89_EXPLICIT => "Becke-Roussel 89 with an explicit inversion of x(y), gamma = 0.8",
            XcFuncId::GGA_XC_KT3 => "Keal and Tozer, version 3",
            XcFuncId::HYB_LDA_XC_BN05 => "Baer and Neuhauser, gamma=1",
            XcFuncId::HYB_GGA_XC_LB07 => "Livshits and Baer, empirical functional also used for IP tuning",
            XcFuncId::LDA_C_PMGB06 => "Long-range LDA correlation functional",
            XcFuncId::GGA_K_GDS08 => "Combined analytical theory with Monte Carlo sampling",
            XcFuncId::GGA_K_GHDS10 => "As GDS08 but for an electron gas with spin",
            XcFuncId::GGA_K_GHDS10R => "Reparametrized GHDS10",
            XcFuncId::GGA_K_TKVLN => "Trickey, Karasiev, and Vela",
            XcFuncId::GGA_K_PBE3 => "Three parameter PBE-like expansion",
            XcFuncId::GGA_K_PBE4 => "Four parameter PBE-like expansion",
            XcFuncId::GGA_K_EXP4 => "Intermediate form between PBE3 and PBE4",
            XcFuncId::HYB_MGGA_XC_B98 => "Becke 98",
            XcFuncId::LDA_XC_TIH => "Neural network LDA from Tozer et al",
            XcFuncId::LDA_X_1D_EXPONENTIAL => "Exchange in 1D for an exponentially screened interaction",
            XcFuncId::GGA_X_SFAT_PBE => "Short-range recipe for PBE functional - Yukawa",
            XcFuncId::MGGA_X_BR89_EXPLICIT_1 => "Becke-Roussel 89 with an explicit inversion of x(y), gamma = 1.0",
            XcFuncId::MGGA_X_REGTPSS => "Regularized TPSS",
            XcFuncId::GGA_X_FD_LB94 => "Functional derivative recovered from the stray LB94 potential",
            XcFuncId::GGA_X_FD_REVLB94 => "Revised FD_LB94",
            XcFuncId::GGA_C_ZVPBELOC => "PBEloc variation with enhanced compatibility with exact exchange",
            XcFuncId::HYB_GGA_XC_APBE0 => "Hybrid based on APBE",
            XcFuncId::HYB_GGA_XC_HAPBE => "Hybrid based in APBE and zvPBEloc",
            XcFuncId::MGGA_X_2D_JS17 => "JS17 meta-GGA for 2D",
            XcFuncId::HYB_GGA_XC_RCAM_B3LYP => "Similar to CAM-B3LYP, but trying to reduce the many-electron self-interaction",
            XcFuncId::HYB_GGA_XC_WC04 => "hybrid fitted to carbon NMR shifts",
            XcFuncId::HYB_GGA_XC_WP04 => "hybrid fitted to proton NMR shifts",
            XcFuncId::GGA_K_LKT => "Luo-Karasiev-Trickey GGA kinetic",
            XcFuncId::HYB_GGA_XC_CAMH_B3LYP => "CAM version of B3LYP, tuned for TDDFT",
            XcFuncId::HYB_GGA_XC_WHPBE0 => "Long-range corrected short-range hybrid PBE (whPBE0) by Shao et al",
            XcFuncId::GGA_K_PBE2 => "Three parameter PBE-like expansion",
            XcFuncId::MGGA_K_L04 => "L0.4 by Laricchia et al",
            XcFuncId::MGGA_K_L06 => "L0.6 by Laricchia et al",
            XcFuncId::GGA_K_VT84F => "VT84F by Karasiev et al",
            XcFuncId::GGA_K_LGAP => "LGAP by Constantin et al",
            XcFuncId::MGGA_K_RDA => "Reduced derivative approximation by Karasiev et al",
            XcFuncId::GGA_X_ITYH_OPTX => "Short-range recipe for OPTX functional",
            XcFuncId::GGA_X_ITYH_PBE => "Short-range recipe for PBE functional",
            XcFuncId::GGA_C_LYPR => "Short-range LYP by Ai, Fang, and Su",
            XcFuncId::HYB_GGA_XC_LC_BLYP_EA => "LC version of BLYP for electron affinities",
            XcFuncId::MGGA_X_REGTM => "Regularized Tao and Mo exchange",
            XcFuncId::MGGA_K_GEA2 => "Second-order gradient expansion",
            XcFuncId::MGGA_K_GEA4 => "Fourth-order gradient expansion",
            XcFuncId::MGGA_K_CSK1 => "mGGA-rev functional by Cancio, Stewart, and Kuna (a=1)",
            XcFuncId::MGGA_K_CSK4 => "mGGA-rev functional by Cancio, Stewart, and Kuna (a=4)",
            XcFuncId::MGGA_K_CSK_LOC1 => "mGGAloc-rev functional by Cancio, Stewart, and Kuna (a=1)",
            XcFuncId::MGGA_K_CSK_LOC4 => "mGGAloc-rev functional by Cancio, Stewart, and Kuna (a=4)",
            XcFuncId::GGA_K_LGAP_GE => "LGAP-GE by Constantin et al",
            XcFuncId::MGGA_K_PC07_OPT => "Reoptimized PC07 by Mejia-Rodriguez and Trickey",
            XcFuncId::GGA_K_TFVW_OPT => "empirically optimized gamma-TFvW form",
            XcFuncId::HYB_GGA_XC_LC_BOP => "LC version of B88",
            XcFuncId::HYB_GGA_XC_LC_PBEOP => "LC version of PBE",
            XcFuncId::MGGA_C_KCISK => "Krieger, Chen, and Kurth",
            XcFuncId::HYB_GGA_XC_LC_BLYPR => "LC version of BLYP with correlation only in the short range",
            XcFuncId::HYB_GGA_XC_MCAM_B3LYP => "Modified CAM-B3LYP by Day, Nguyen and Pachter",
            XcFuncId::LDA_X_YUKAWA => "Short-range LDA exchange with Yukawa attenuation",
            XcFuncId::MGGA_C_R2SCAN01 => "Re-regularized SCAN correlation with larger value for eta",
            XcFuncId::MGGA_C_RMGGAC => "Revised correlation energy for MGGAC exchange functional",
            XcFuncId::MGGA_X_MCML => "MCML exchange",
            XcFuncId::MGGA_X_R2SCAN01 => "Re-regularized SCAN exchange by Furness et al with larger value for eta",
            XcFuncId::HYB_GGA_X_CAM_S12G => "Swart 2012 range-separated hybrid GGA exchange",
            XcFuncId::HYB_GGA_X_CAM_S12H => "Swart 2012 range-separated hybrid GGA exchange",
            XcFuncId::MGGA_X_RPPSCAN => "r++SCAN: rSCAN with uniform density limit and coordinate scaling behavior",
            XcFuncId::MGGA_C_RPPSCAN => "r++SCAN: rSCAN with uniform density limit and coordinate scaling behavior",
            XcFuncId::MGGA_X_R4SCAN => "r$^{4}$SCAN, a functional that satisfies the same exact constraints that SCAN does",
            XcFuncId::MGGA_X_VCML => "Exchange part of VCML-rVV10 by Trepte and Voss",
            XcFuncId::MGGA_XC_VCML_RVV10 => "VCML-rVV10 by Trepte and Voss",
            XcFuncId::HYB_LDA_X_ERF => "Long-range corrected functional based on short-range LDA exchange (erfc)",
            XcFuncId::LDA_C_PW_ERF => "Short ranged correlation LDA (erfc)",
            XcFuncId::GGA_X_PBE_ERF_GWS => "Short ranged PBE exchange (erfc)",
            XcFuncId::HYB_GGA_X_PBE_ERF_GWS => "Short-range PBE (GWS) exchange (erfc) + long-range exact exchange",
            XcFuncId::GGA_C_PBE_ERF_GWS => "Short ranged PBE correlation (erfc)",
            XcFuncId::HYB_MGGA_XC_GAS22 => "Google Accelerated Science 22",
            XcFuncId::HYB_MGGA_XC_R2SCANH => "r2SCANh: r2SCAN hybrid like TPSSh with 10% exact exchange",
            XcFuncId::HYB_MGGA_XC_R2SCAN0 => "r2SCAN0: r2SCAN hybrid like PBE0 with 25% exact exchange",
            XcFuncId::HYB_MGGA_XC_R2SCAN50 => "r2SCAN50: r2SCAN hybrid like PBE50 with 50% exact exchange",
            XcFuncId::HYB_MGGA_X_WR2SCAN => "Range-separated re-regularized SCAN exchange by Wittmann et al",
            XcFuncId::HYB_GGA_XC_CAM_PBEH => "CAM hybrid screened exchange PBE version",
            XcFuncId::HYB_GGA_XC_CAMY_PBEH => "CAMY hybrid screened exchange PBE version",
            XcFuncId::LDA_C_UPW92 => "Ruggeri, Rios, and Alavi unrestricted fit",
            XcFuncId::LDA_C_RPW92 => "Ruggeri, Rios, and Alavi restricted fit",
            XcFuncId::MGGA_X_TLDA => "LDA-type exchange with tau-dependent potential",
            XcFuncId::MGGA_X_EDMGGA => "Tao 2001",
            XcFuncId::MGGA_X_GDME_NV => "Generalized density-matrix with a=1/2",
            XcFuncId::MGGA_X_RLDA => "Reparametrized local-density approximation",
            XcFuncId::MGGA_X_GDME_0 => "Generalized density-matrix with a=0",
            XcFuncId::MGGA_X_GDME_KOS => "Generalized density-matrix with a=0.00638",
            XcFuncId::MGGA_X_GDME_VT => "Varied-terms (VT) mGGA of Koehl, Odom, and Scuseria",
            XcFuncId::LDA_X_SLOC => "simple local model for Slater potential",
            XcFuncId::MGGA_X_REVTM => "revised Tao and Mo 2016 exchange",
            XcFuncId::MGGA_C_REVTM => "revised Tao and Mo 2016 exchange",
            XcFuncId::HYB_MGGA_XC_EDMGGAH => "EDMGGA hybrid",
            XcFuncId::MGGA_X_MBRXC_BG => "Modified Becke-Roussel for band gaps - cuspless hole",
            XcFuncId::MGGA_X_MBRXH_BG => "Modified Becke-Roussel for band gaps - hydrogen hole",
            XcFuncId::MGGA_X_HLTA => "Half-and-half meta-LDAized LDA exchange by Lehtola and Marques",
            XcFuncId::MGGA_C_HLTAPW => "Half-and-half meta-LDAized PW correlation by Lehtola and Marques",
            XcFuncId::MGGA_X_SCANL => "Deorbitalized SCAN (SCAN-L) exchange",
            XcFuncId::MGGA_X_REVSCANL => "Deorbitalized revised SCAN (revSCAN-L) exchange",
            XcFuncId::MGGA_C_SCANL => "Deorbitalized SCAN (SCAN-L) correlation",
            XcFuncId::MGGA_C_SCANL_RVV10 => "SCAN-L + rVV10 correlation",
            XcFuncId::MGGA_C_SCANL_VV10 => "SCAN-L + VV10 correlation",
            XcFuncId::HYB_MGGA_X_JS18 => "Jana and Samal 2018, screened range-separated TM exchange",
            XcFuncId::HYB_MGGA_X_PJS18 => "Patra, Jana and Samal 2018, screened range-separated TM exchange",
            XcFuncId::MGGA_X_TASK => "TASK exchange of Aschebrock and Kuemmel",
            XcFuncId::MGGA_X_MGGAC => "MGGAC exchange of Patra et al",
            XcFuncId::GGA_C_MGGAC => "beta fitted to LC20 to be used with MGGAC",
            XcFuncId::MGGA_X_MBR => "modified Becke-Roussel by Patra et al",
            XcFuncId::MGGA_X_R2SCANL => "Deorbitalized re-regularized SCAN (r2SCAN-L) exchange",
            XcFuncId::MGGA_C_R2SCANL => "Deorbitalized re-regularized SCAN (r2SCAN-L) correlation",
            XcFuncId::HYB_MGGA_XC_LC_TMLYP => "Long-range corrected TM-LYP by Jana et al",
            XcFuncId::MGGA_X_MTASK => "modified TASK exchange",
            XcFuncId::GGA_X_Q1D => "Functional for quasi-1D systems",
            XcFuncId::MGGA_X_KTBM_0 => "KTBM learned exchange - 0",
            XcFuncId::MGGA_X_KTBM_1 => "KTBM learned exchange - 1",
            XcFuncId::MGGA_X_KTBM_2 => "KTBM learned exchange - 2",
            XcFuncId::MGGA_X_KTBM_3 => "KTBM learned exchange - 3",
            XcFuncId::MGGA_X_KTBM_4 => "KTBM learned exchange - 4",
            XcFuncId::MGGA_X_KTBM_5 => "KTBM learned exchange - 5",
            XcFuncId::MGGA_X_KTBM_6 => "KTBM learned exchange - 6",
            XcFuncId::MGGA_X_KTBM_7 => "KTBM learned exchange - 7",
            XcFuncId::MGGA_X_KTBM_8 => "KTBM learned exchange - 8",
            XcFuncId::MGGA_X_KTBM_9 => "KTBM learned exchange - 9",
            XcFuncId::MGGA_X_KTBM_10 => "KTBM learned exchange - 10",
            XcFuncId::MGGA_X_KTBM_11 => "KTBM learned exchange - 11",
            XcFuncId::MGGA_X_KTBM_12 => "KTBM learned exchange - 12",
            XcFuncId::MGGA_X_KTBM_13 => "KTBM learned exchange - 13",
            XcFuncId::MGGA_X_KTBM_14 => "KTBM learned exchange - 14",
            XcFuncId::MGGA_X_KTBM_15 => "KTBM learned exchange - 15",
            XcFuncId::MGGA_X_KTBM_16 => "KTBM learned exchange - 16",
            XcFuncId::MGGA_X_KTBM_17 => "KTBM learned exchange - 17",
            XcFuncId::MGGA_X_KTBM_18 => "KTBM learned exchange - 18",
            XcFuncId::MGGA_X_KTBM_19 => "KTBM learned exchange - 19",
            XcFuncId::MGGA_X_KTBM_20 => "KTBM learned exchange - 20",
            XcFuncId::MGGA_X_KTBM_21 => "KTBM learned exchange - 21",
            XcFuncId::MGGA_X_KTBM_22 => "KTBM learned exchange - 22",
            XcFuncId::MGGA_X_KTBM_23 => "KTBM learned exchange - 23",
            XcFuncId::MGGA_X_KTBM_24 => "KTBM learned exchange - 24",
            XcFuncId::MGGA_X_KTBM_GAP => "KTBM learned exchange - GAP",
            XcFuncId::MGGA_X_MSPBEL => "MS-PBEl, a PBE-like meta-GGA exchange",
            XcFuncId::MGGA_X_RMSPBEL => "regularized MS-PBEl",
            XcFuncId::MGGA_X_MSRPBEL => "MS-RPBEl, a RPBE-like meta-GGA exchange",
            XcFuncId::MGGA_X_RMSRPBEL => "regularized MS-RPBEl",
            XcFuncId::MGGA_X_MSB86BL => "MS-B86bl, a B86b-like meta-GGA exchange",
            XcFuncId::MGGA_X_RMSB86BL => "regularized MS-B86bl",
            XcFuncId::HYB_MGGA_X_PI_M06_2X_DL => "Dispersionless physically-informed Minnesota M06-2X hybrid exchange functional",
            XcFuncId::MGGA_C_PI_M06_2X_DL => "Dispersionless physically-informed Minnesota M06-2X correlation functional",
            XcFuncId::HYB_MGGA_X_PI_M06_2X => "Physically-informed Minnesota M06-2X hybrid exchange functional",
            XcFuncId::MGGA_C_PI_M06_2X => "Physically-informed Minnesota M06-2X correlation functional",
        }
    }
}

impl std::fmt::Display for XcFuncId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for XcFuncId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try direct name match (case-insensitive)
        // Accept both "LDA_X" and "XC_LDA_X" forms
        let lookup = s.trim().to_uppercase();
        let lookup = lookup.strip_prefix("XC_").unwrap_or(&lookup);
        if lookup == "LDA_X" {
            return Ok(XcFuncId::LDA_X);
        }
        if lookup == "LDA_C_WIGNER" {
            return Ok(XcFuncId::LDA_C_WIGNER);
        }
        if lookup == "LDA_C_RPA" {
            return Ok(XcFuncId::LDA_C_RPA);
        }
        if lookup == "LDA_C_HL" {
            return Ok(XcFuncId::LDA_C_HL);
        }
        if lookup == "LDA_C_GL" {
            return Ok(XcFuncId::LDA_C_GL);
        }
        if lookup == "LDA_C_XALPHA" {
            return Ok(XcFuncId::LDA_C_XALPHA);
        }
        if lookup == "LDA_C_VWN" {
            return Ok(XcFuncId::LDA_C_VWN);
        }
        if lookup == "LDA_C_VWN_RPA" {
            return Ok(XcFuncId::LDA_C_VWN_RPA);
        }
        if lookup == "LDA_C_PZ" {
            return Ok(XcFuncId::LDA_C_PZ);
        }
        if lookup == "LDA_C_PZ_MOD" {
            return Ok(XcFuncId::LDA_C_PZ_MOD);
        }
        if lookup == "LDA_C_OB_PZ" {
            return Ok(XcFuncId::LDA_C_OB_PZ);
        }
        if lookup == "LDA_C_PW" {
            return Ok(XcFuncId::LDA_C_PW);
        }
        if lookup == "LDA_C_PW_MOD" {
            return Ok(XcFuncId::LDA_C_PW_MOD);
        }
        if lookup == "LDA_C_OB_PW" {
            return Ok(XcFuncId::LDA_C_OB_PW);
        }
        if lookup == "LDA_C_2D_AMGB" {
            return Ok(XcFuncId::LDA_C_2D_AMGB);
        }
        if lookup == "LDA_C_2D_PRM" {
            return Ok(XcFuncId::LDA_C_2D_PRM);
        }
        if lookup == "LDA_C_VBH" {
            return Ok(XcFuncId::LDA_C_VBH);
        }
        if lookup == "LDA_C_1D_CSS" {
            return Ok(XcFuncId::LDA_C_1D_CSS);
        }
        if lookup == "LDA_X_2D" {
            return Ok(XcFuncId::LDA_X_2D);
        }
        if lookup == "LDA_XC_TETER93" {
            return Ok(XcFuncId::LDA_XC_TETER93);
        }
        if lookup == "LDA_X_1D_SOFT" {
            return Ok(XcFuncId::LDA_X_1D_SOFT);
        }
        if lookup == "LDA_C_ML1" {
            return Ok(XcFuncId::LDA_C_ML1);
        }
        if lookup == "LDA_C_ML2" {
            return Ok(XcFuncId::LDA_C_ML2);
        }
        if lookup == "LDA_C_GOMBAS" {
            return Ok(XcFuncId::LDA_C_GOMBAS);
        }
        if lookup == "LDA_C_PW_RPA" {
            return Ok(XcFuncId::LDA_C_PW_RPA);
        }
        if lookup == "LDA_C_1D_LOOS" {
            return Ok(XcFuncId::LDA_C_1D_LOOS);
        }
        if lookup == "LDA_C_RC04" {
            return Ok(XcFuncId::LDA_C_RC04);
        }
        if lookup == "LDA_C_VWN_1" {
            return Ok(XcFuncId::LDA_C_VWN_1);
        }
        if lookup == "LDA_C_VWN_2" {
            return Ok(XcFuncId::LDA_C_VWN_2);
        }
        if lookup == "LDA_C_VWN_3" {
            return Ok(XcFuncId::LDA_C_VWN_3);
        }
        if lookup == "LDA_C_VWN_4" {
            return Ok(XcFuncId::LDA_C_VWN_4);
        }
        if lookup == "GGA_X_GAM" {
            return Ok(XcFuncId::GGA_X_GAM);
        }
        if lookup == "GGA_C_GAM" {
            return Ok(XcFuncId::GGA_C_GAM);
        }
        if lookup == "GGA_X_HCTH_A" {
            return Ok(XcFuncId::GGA_X_HCTH_A);
        }
        if lookup == "GGA_X_EV93" {
            return Ok(XcFuncId::GGA_X_EV93);
        }
        if lookup == "HYB_MGGA_X_DLDF" {
            return Ok(XcFuncId::HYB_MGGA_X_DLDF);
        }
        if lookup == "MGGA_C_DLDF" {
            return Ok(XcFuncId::MGGA_C_DLDF);
        }
        if lookup == "GGA_X_BCGP" {
            return Ok(XcFuncId::GGA_X_BCGP);
        }
        if lookup == "GGA_C_ACGGA" {
            return Ok(XcFuncId::GGA_C_ACGGA);
        }
        if lookup == "GGA_X_LAMBDA_OC2_N" {
            return Ok(XcFuncId::GGA_X_LAMBDA_OC2_N);
        }
        if lookup == "GGA_X_B86_R" {
            return Ok(XcFuncId::GGA_X_B86_R);
        }
        if lookup == "MGGA_XC_ZLP" {
            return Ok(XcFuncId::MGGA_XC_ZLP);
        }
        if lookup == "LDA_XC_ZLP" {
            return Ok(XcFuncId::LDA_XC_ZLP);
        }
        if lookup == "GGA_X_LAMBDA_CH_N" {
            return Ok(XcFuncId::GGA_X_LAMBDA_CH_N);
        }
        if lookup == "GGA_X_LAMBDA_LO_N" {
            return Ok(XcFuncId::GGA_X_LAMBDA_LO_N);
        }
        if lookup == "GGA_X_HJS_B88_V2" {
            return Ok(XcFuncId::GGA_X_HJS_B88_V2);
        }
        if lookup == "GGA_C_Q2D" {
            return Ok(XcFuncId::GGA_C_Q2D);
        }
        if lookup == "GGA_X_Q2D" {
            return Ok(XcFuncId::GGA_X_Q2D);
        }
        if lookup == "GGA_X_PBE_MOL" {
            return Ok(XcFuncId::GGA_X_PBE_MOL);
        }
        if lookup == "LDA_K_TF" {
            return Ok(XcFuncId::LDA_K_TF);
        }
        if lookup == "LDA_K_LP" {
            return Ok(XcFuncId::LDA_K_LP);
        }
        if lookup == "GGA_K_TFVW" {
            return Ok(XcFuncId::GGA_K_TFVW);
        }
        if lookup == "GGA_K_REVAPBEINT" {
            return Ok(XcFuncId::GGA_K_REVAPBEINT);
        }
        if lookup == "GGA_K_APBEINT" {
            return Ok(XcFuncId::GGA_K_APBEINT);
        }
        if lookup == "GGA_K_REVAPBE" {
            return Ok(XcFuncId::GGA_K_REVAPBE);
        }
        if lookup == "GGA_X_AK13" {
            return Ok(XcFuncId::GGA_X_AK13);
        }
        if lookup == "GGA_K_MEYER" {
            return Ok(XcFuncId::GGA_K_MEYER);
        }
        if lookup == "GGA_X_LV_RPW86" {
            return Ok(XcFuncId::GGA_X_LV_RPW86);
        }
        if lookup == "GGA_X_PBE_TCA" {
            return Ok(XcFuncId::GGA_X_PBE_TCA);
        }
        if lookup == "GGA_X_PBEINT" {
            return Ok(XcFuncId::GGA_X_PBEINT);
        }
        if lookup == "GGA_C_ZPBEINT" {
            return Ok(XcFuncId::GGA_C_ZPBEINT);
        }
        if lookup == "GGA_C_PBEINT" {
            return Ok(XcFuncId::GGA_C_PBEINT);
        }
        if lookup == "GGA_C_ZPBESOL" {
            return Ok(XcFuncId::GGA_C_ZPBESOL);
        }
        if lookup == "MGGA_XC_OTPSS_D" {
            return Ok(XcFuncId::MGGA_XC_OTPSS_D);
        }
        if lookup == "GGA_XC_OPBE_D" {
            return Ok(XcFuncId::GGA_XC_OPBE_D);
        }
        if lookup == "GGA_XC_OPWLYP_D" {
            return Ok(XcFuncId::GGA_XC_OPWLYP_D);
        }
        if lookup == "GGA_XC_OBLYP_D" {
            return Ok(XcFuncId::GGA_XC_OBLYP_D);
        }
        if lookup == "GGA_X_VMT84_GE" {
            return Ok(XcFuncId::GGA_X_VMT84_GE);
        }
        if lookup == "GGA_X_VMT84_PBE" {
            return Ok(XcFuncId::GGA_X_VMT84_PBE);
        }
        if lookup == "GGA_X_VMT_GE" {
            return Ok(XcFuncId::GGA_X_VMT_GE);
        }
        if lookup == "GGA_X_VMT_PBE" {
            return Ok(XcFuncId::GGA_X_VMT_PBE);
        }
        if lookup == "MGGA_C_CS" {
            return Ok(XcFuncId::MGGA_C_CS);
        }
        if lookup == "MGGA_C_MN12_SX" {
            return Ok(XcFuncId::MGGA_C_MN12_SX);
        }
        if lookup == "MGGA_C_MN12_L" {
            return Ok(XcFuncId::MGGA_C_MN12_L);
        }
        if lookup == "MGGA_C_M11_L" {
            return Ok(XcFuncId::MGGA_C_M11_L);
        }
        if lookup == "MGGA_C_M11" {
            return Ok(XcFuncId::MGGA_C_M11);
        }
        if lookup == "MGGA_C_M08_SO" {
            return Ok(XcFuncId::MGGA_C_M08_SO);
        }
        if lookup == "MGGA_C_M08_HX" {
            return Ok(XcFuncId::MGGA_C_M08_HX);
        }
        if lookup == "GGA_C_N12_SX" {
            return Ok(XcFuncId::GGA_C_N12_SX);
        }
        if lookup == "GGA_C_N12" {
            return Ok(XcFuncId::GGA_C_N12);
        }
        if lookup == "HYB_GGA_X_N12_SX" {
            return Ok(XcFuncId::HYB_GGA_X_N12_SX);
        }
        if lookup == "GGA_X_N12" {
            return Ok(XcFuncId::GGA_X_N12);
        }
        if lookup == "GGA_C_REGTPSS" {
            return Ok(XcFuncId::GGA_C_REGTPSS);
        }
        if lookup == "GGA_C_OP_XALPHA" {
            return Ok(XcFuncId::GGA_C_OP_XALPHA);
        }
        if lookup == "GGA_C_OP_G96" {
            return Ok(XcFuncId::GGA_C_OP_G96);
        }
        if lookup == "GGA_C_OP_PBE" {
            return Ok(XcFuncId::GGA_C_OP_PBE);
        }
        if lookup == "GGA_C_OP_B88" {
            return Ok(XcFuncId::GGA_C_OP_B88);
        }
        if lookup == "GGA_C_FT97" {
            return Ok(XcFuncId::GGA_C_FT97);
        }
        if lookup == "GGA_C_SPBE" {
            return Ok(XcFuncId::GGA_C_SPBE);
        }
        if lookup == "GGA_X_SSB_SW" {
            return Ok(XcFuncId::GGA_X_SSB_SW);
        }
        if lookup == "GGA_X_SSB" {
            return Ok(XcFuncId::GGA_X_SSB);
        }
        if lookup == "GGA_X_SSB_D" {
            return Ok(XcFuncId::GGA_X_SSB_D);
        }
        if lookup == "GGA_XC_HCTH_407P" {
            return Ok(XcFuncId::GGA_XC_HCTH_407P);
        }
        if lookup == "GGA_XC_HCTH_P76" {
            return Ok(XcFuncId::GGA_XC_HCTH_P76);
        }
        if lookup == "GGA_XC_HCTH_P14" {
            return Ok(XcFuncId::GGA_XC_HCTH_P14);
        }
        if lookup == "GGA_XC_B97_GGA1" {
            return Ok(XcFuncId::GGA_XC_B97_GGA1);
        }
        if lookup == "GGA_C_HCTH_A" {
            return Ok(XcFuncId::GGA_C_HCTH_A);
        }
        if lookup == "GGA_X_BPCCAC" {
            return Ok(XcFuncId::GGA_X_BPCCAC);
        }
        if lookup == "GGA_C_REVTCA" {
            return Ok(XcFuncId::GGA_C_REVTCA);
        }
        if lookup == "GGA_C_TCA" {
            return Ok(XcFuncId::GGA_C_TCA);
        }
        if lookup == "GGA_X_PBE" {
            return Ok(XcFuncId::GGA_X_PBE);
        }
        if lookup == "GGA_X_PBE_R" {
            return Ok(XcFuncId::GGA_X_PBE_R);
        }
        if lookup == "GGA_X_B86" {
            return Ok(XcFuncId::GGA_X_B86);
        }
        if lookup == "HYB_LDA_XC_B93" {
            return Ok(XcFuncId::HYB_LDA_XC_B93);
        }
        if lookup == "GGA_X_B86_MGC" {
            return Ok(XcFuncId::GGA_X_B86_MGC);
        }
        if lookup == "GGA_X_B88" {
            return Ok(XcFuncId::GGA_X_B88);
        }
        if lookup == "GGA_X_G96" {
            return Ok(XcFuncId::GGA_X_G96);
        }
        if lookup == "GGA_X_PW86" {
            return Ok(XcFuncId::GGA_X_PW86);
        }
        if lookup == "GGA_X_PW91" {
            return Ok(XcFuncId::GGA_X_PW91);
        }
        if lookup == "GGA_X_OPTX" {
            return Ok(XcFuncId::GGA_X_OPTX);
        }
        if lookup == "GGA_X_DK87_R1" {
            return Ok(XcFuncId::GGA_X_DK87_R1);
        }
        if lookup == "GGA_X_DK87_R2" {
            return Ok(XcFuncId::GGA_X_DK87_R2);
        }
        if lookup == "GGA_X_LG93" {
            return Ok(XcFuncId::GGA_X_LG93);
        }
        if lookup == "GGA_X_FT97_A" {
            return Ok(XcFuncId::GGA_X_FT97_A);
        }
        if lookup == "GGA_X_FT97_B" {
            return Ok(XcFuncId::GGA_X_FT97_B);
        }
        if lookup == "GGA_X_PBE_SOL" {
            return Ok(XcFuncId::GGA_X_PBE_SOL);
        }
        if lookup == "GGA_X_RPBE" {
            return Ok(XcFuncId::GGA_X_RPBE);
        }
        if lookup == "GGA_X_WC" {
            return Ok(XcFuncId::GGA_X_WC);
        }
        if lookup == "GGA_X_MPW91" {
            return Ok(XcFuncId::GGA_X_MPW91);
        }
        if lookup == "GGA_X_AM05" {
            return Ok(XcFuncId::GGA_X_AM05);
        }
        if lookup == "GGA_X_PBEA" {
            return Ok(XcFuncId::GGA_X_PBEA);
        }
        if lookup == "GGA_X_MPBE" {
            return Ok(XcFuncId::GGA_X_MPBE);
        }
        if lookup == "GGA_X_XPBE" {
            return Ok(XcFuncId::GGA_X_XPBE);
        }
        if lookup == "GGA_X_2D_B86_MGC" {
            return Ok(XcFuncId::GGA_X_2D_B86_MGC);
        }
        if lookup == "GGA_X_BAYESIAN" {
            return Ok(XcFuncId::GGA_X_BAYESIAN);
        }
        if lookup == "GGA_X_PBE_JSJR" {
            return Ok(XcFuncId::GGA_X_PBE_JSJR);
        }
        if lookup == "GGA_X_2D_B88" {
            return Ok(XcFuncId::GGA_X_2D_B88);
        }
        if lookup == "GGA_X_2D_B86" {
            return Ok(XcFuncId::GGA_X_2D_B86);
        }
        if lookup == "GGA_X_2D_PBE" {
            return Ok(XcFuncId::GGA_X_2D_PBE);
        }
        if lookup == "GGA_C_PBE" {
            return Ok(XcFuncId::GGA_C_PBE);
        }
        if lookup == "GGA_C_LYP" {
            return Ok(XcFuncId::GGA_C_LYP);
        }
        if lookup == "GGA_C_P86" {
            return Ok(XcFuncId::GGA_C_P86);
        }
        if lookup == "GGA_C_PBE_SOL" {
            return Ok(XcFuncId::GGA_C_PBE_SOL);
        }
        if lookup == "GGA_C_PW91" {
            return Ok(XcFuncId::GGA_C_PW91);
        }
        if lookup == "GGA_C_AM05" {
            return Ok(XcFuncId::GGA_C_AM05);
        }
        if lookup == "GGA_C_XPBE" {
            return Ok(XcFuncId::GGA_C_XPBE);
        }
        if lookup == "GGA_C_LM" {
            return Ok(XcFuncId::GGA_C_LM);
        }
        if lookup == "GGA_C_PBE_JRGX" {
            return Ok(XcFuncId::GGA_C_PBE_JRGX);
        }
        if lookup == "GGA_X_OPTB88_VDW" {
            return Ok(XcFuncId::GGA_X_OPTB88_VDW);
        }
        if lookup == "GGA_X_PBEK1_VDW" {
            return Ok(XcFuncId::GGA_X_PBEK1_VDW);
        }
        if lookup == "GGA_X_OPTPBE_VDW" {
            return Ok(XcFuncId::GGA_X_OPTPBE_VDW);
        }
        if lookup == "GGA_X_RGE2" {
            return Ok(XcFuncId::GGA_X_RGE2);
        }
        if lookup == "GGA_C_RGE2" {
            return Ok(XcFuncId::GGA_C_RGE2);
        }
        if lookup == "GGA_X_RPW86" {
            return Ok(XcFuncId::GGA_X_RPW86);
        }
        if lookup == "GGA_X_KT1" {
            return Ok(XcFuncId::GGA_X_KT1);
        }
        if lookup == "GGA_XC_KT2" {
            return Ok(XcFuncId::GGA_XC_KT2);
        }
        if lookup == "GGA_C_WL" {
            return Ok(XcFuncId::GGA_C_WL);
        }
        if lookup == "GGA_C_WI" {
            return Ok(XcFuncId::GGA_C_WI);
        }
        if lookup == "GGA_X_MB88" {
            return Ok(XcFuncId::GGA_X_MB88);
        }
        if lookup == "GGA_X_SOGGA" {
            return Ok(XcFuncId::GGA_X_SOGGA);
        }
        if lookup == "GGA_X_SOGGA11" {
            return Ok(XcFuncId::GGA_X_SOGGA11);
        }
        if lookup == "GGA_C_SOGGA11" {
            return Ok(XcFuncId::GGA_C_SOGGA11);
        }
        if lookup == "GGA_C_WI0" {
            return Ok(XcFuncId::GGA_C_WI0);
        }
        if lookup == "GGA_XC_TH1" {
            return Ok(XcFuncId::GGA_XC_TH1);
        }
        if lookup == "GGA_XC_TH2" {
            return Ok(XcFuncId::GGA_XC_TH2);
        }
        if lookup == "GGA_XC_TH3" {
            return Ok(XcFuncId::GGA_XC_TH3);
        }
        if lookup == "GGA_XC_TH4" {
            return Ok(XcFuncId::GGA_XC_TH4);
        }
        if lookup == "GGA_X_C09X" {
            return Ok(XcFuncId::GGA_X_C09X);
        }
        if lookup == "GGA_C_SOGGA11_X" {
            return Ok(XcFuncId::GGA_C_SOGGA11_X);
        }
        if lookup == "GGA_X_LB" {
            return Ok(XcFuncId::GGA_X_LB);
        }
        if lookup == "GGA_XC_HCTH_93" {
            return Ok(XcFuncId::GGA_XC_HCTH_93);
        }
        if lookup == "GGA_XC_HCTH_120" {
            return Ok(XcFuncId::GGA_XC_HCTH_120);
        }
        if lookup == "GGA_XC_HCTH_147" {
            return Ok(XcFuncId::GGA_XC_HCTH_147);
        }
        if lookup == "GGA_XC_HCTH_407" {
            return Ok(XcFuncId::GGA_XC_HCTH_407);
        }
        if lookup == "GGA_XC_EDF1" {
            return Ok(XcFuncId::GGA_XC_EDF1);
        }
        if lookup == "GGA_XC_XLYP" {
            return Ok(XcFuncId::GGA_XC_XLYP);
        }
        if lookup == "GGA_XC_KT1" {
            return Ok(XcFuncId::GGA_XC_KT1);
        }
        if lookup == "GGA_X_LSPBE" {
            return Ok(XcFuncId::GGA_X_LSPBE);
        }
        if lookup == "GGA_X_LSRPBE" {
            return Ok(XcFuncId::GGA_X_LSRPBE);
        }
        if lookup == "GGA_XC_B97_D" {
            return Ok(XcFuncId::GGA_XC_B97_D);
        }
        if lookup == "GGA_X_OPTB86B_VDW" {
            return Ok(XcFuncId::GGA_X_OPTB86B_VDW);
        }
        if lookup == "MGGA_C_REVM11" {
            return Ok(XcFuncId::MGGA_C_REVM11);
        }
        if lookup == "GGA_XC_PBE1W" {
            return Ok(XcFuncId::GGA_XC_PBE1W);
        }
        if lookup == "GGA_XC_MPWLYP1W" {
            return Ok(XcFuncId::GGA_XC_MPWLYP1W);
        }
        if lookup == "GGA_XC_PBELYP1W" {
            return Ok(XcFuncId::GGA_XC_PBELYP1W);
        }
        if lookup == "GGA_C_ACGGAP" {
            return Ok(XcFuncId::GGA_C_ACGGAP);
        }
        if lookup == "HYB_LDA_XC_LDA0" {
            return Ok(XcFuncId::HYB_LDA_XC_LDA0);
        }
        if lookup == "HYB_LDA_XC_CAM_LDA0" {
            return Ok(XcFuncId::HYB_LDA_XC_CAM_LDA0);
        }
        if lookup == "GGA_X_B88_6311G" {
            return Ok(XcFuncId::GGA_X_B88_6311G);
        }
        if lookup == "GGA_X_NCAP" {
            return Ok(XcFuncId::GGA_X_NCAP);
        }
        if lookup == "GGA_XC_NCAP" {
            return Ok(XcFuncId::GGA_XC_NCAP);
        }
        if lookup == "GGA_X_LBM" {
            return Ok(XcFuncId::GGA_X_LBM);
        }
        if lookup == "GGA_X_OL2" {
            return Ok(XcFuncId::GGA_X_OL2);
        }
        if lookup == "GGA_X_APBE" {
            return Ok(XcFuncId::GGA_X_APBE);
        }
        if lookup == "GGA_K_APBE" {
            return Ok(XcFuncId::GGA_K_APBE);
        }
        if lookup == "GGA_C_APBE" {
            return Ok(XcFuncId::GGA_C_APBE);
        }
        if lookup == "GGA_K_TW1" {
            return Ok(XcFuncId::GGA_K_TW1);
        }
        if lookup == "GGA_K_TW2" {
            return Ok(XcFuncId::GGA_K_TW2);
        }
        if lookup == "GGA_K_TW3" {
            return Ok(XcFuncId::GGA_K_TW3);
        }
        if lookup == "GGA_K_TW4" {
            return Ok(XcFuncId::GGA_K_TW4);
        }
        if lookup == "GGA_X_HTBS" {
            return Ok(XcFuncId::GGA_X_HTBS);
        }
        if lookup == "GGA_X_AIRY" {
            return Ok(XcFuncId::GGA_X_AIRY);
        }
        if lookup == "GGA_X_LAG" {
            return Ok(XcFuncId::GGA_X_LAG);
        }
        if lookup == "GGA_XC_MOHLYP" {
            return Ok(XcFuncId::GGA_XC_MOHLYP);
        }
        if lookup == "GGA_XC_MOHLYP2" {
            return Ok(XcFuncId::GGA_XC_MOHLYP2);
        }
        if lookup == "LDA_XC_TH_FL" {
            return Ok(XcFuncId::LDA_XC_TH_FL);
        }
        if lookup == "GGA_XC_TH_FC" {
            return Ok(XcFuncId::GGA_XC_TH_FC);
        }
        if lookup == "GGA_XC_TH_FCFO" {
            return Ok(XcFuncId::GGA_XC_TH_FCFO);
        }
        if lookup == "GGA_XC_TH_FCO" {
            return Ok(XcFuncId::GGA_XC_TH_FCO);
        }
        if lookup == "GGA_C_OPTC" {
            return Ok(XcFuncId::GGA_C_OPTC);
        }
        if lookup == "MGGA_X_LTA" {
            return Ok(XcFuncId::MGGA_X_LTA);
        }
        if lookup == "MGGA_X_TPSS" {
            return Ok(XcFuncId::MGGA_X_TPSS);
        }
        if lookup == "MGGA_X_M06_L" {
            return Ok(XcFuncId::MGGA_X_M06_L);
        }
        if lookup == "MGGA_X_GVT4" {
            return Ok(XcFuncId::MGGA_X_GVT4);
        }
        if lookup == "MGGA_X_TAU_HCTH" {
            return Ok(XcFuncId::MGGA_X_TAU_HCTH);
        }
        if lookup == "MGGA_X_BR89" {
            return Ok(XcFuncId::MGGA_X_BR89);
        }
        if lookup == "MGGA_X_BJ06" {
            return Ok(XcFuncId::MGGA_X_BJ06);
        }
        if lookup == "MGGA_X_TB09" {
            return Ok(XcFuncId::MGGA_X_TB09);
        }
        if lookup == "MGGA_X_RPP09" {
            return Ok(XcFuncId::MGGA_X_RPP09);
        }
        if lookup == "MGGA_X_2D_PRHG07" {
            return Ok(XcFuncId::MGGA_X_2D_PRHG07);
        }
        if lookup == "MGGA_X_2D_PRHG07_PRP10" {
            return Ok(XcFuncId::MGGA_X_2D_PRHG07_PRP10);
        }
        if lookup == "MGGA_X_REVTPSS" {
            return Ok(XcFuncId::MGGA_X_REVTPSS);
        }
        if lookup == "MGGA_X_PKZB" {
            return Ok(XcFuncId::MGGA_X_PKZB);
        }
        if lookup == "MGGA_X_BR89_1" {
            return Ok(XcFuncId::MGGA_X_BR89_1);
        }
        if lookup == "GGA_X_ECMV92" {
            return Ok(XcFuncId::GGA_X_ECMV92);
        }
        if lookup == "GGA_C_PBE_VWN" {
            return Ok(XcFuncId::GGA_C_PBE_VWN);
        }
        if lookup == "GGA_C_P86_FT" {
            return Ok(XcFuncId::GGA_C_P86_FT);
        }
        if lookup == "GGA_K_RATIONAL_P" {
            return Ok(XcFuncId::GGA_K_RATIONAL_P);
        }
        if lookup == "GGA_K_PG1" {
            return Ok(XcFuncId::GGA_K_PG1);
        }
        if lookup == "MGGA_K_PGSL025" {
            return Ok(XcFuncId::MGGA_K_PGSL025);
        }
        if lookup == "MGGA_X_MS0" {
            return Ok(XcFuncId::MGGA_X_MS0);
        }
        if lookup == "MGGA_X_MS1" {
            return Ok(XcFuncId::MGGA_X_MS1);
        }
        if lookup == "MGGA_X_MS2" {
            return Ok(XcFuncId::MGGA_X_MS2);
        }
        if lookup == "HYB_MGGA_X_MS2H" {
            return Ok(XcFuncId::HYB_MGGA_X_MS2H);
        }
        if lookup == "MGGA_X_TH" {
            return Ok(XcFuncId::MGGA_X_TH);
        }
        if lookup == "MGGA_X_M11_L" {
            return Ok(XcFuncId::MGGA_X_M11_L);
        }
        if lookup == "MGGA_X_MN12_L" {
            return Ok(XcFuncId::MGGA_X_MN12_L);
        }
        if lookup == "MGGA_X_MS2_REV" {
            return Ok(XcFuncId::MGGA_X_MS2_REV);
        }
        if lookup == "MGGA_XC_CC06" {
            return Ok(XcFuncId::MGGA_XC_CC06);
        }
        if lookup == "MGGA_X_GP86" {
            return Ok(XcFuncId::MGGA_X_GP86);
        }
        if lookup == "MGGA_C_TPSS" {
            return Ok(XcFuncId::MGGA_C_TPSS);
        }
        if lookup == "MGGA_C_VSXC" {
            return Ok(XcFuncId::MGGA_C_VSXC);
        }
        if lookup == "MGGA_C_M06_L" {
            return Ok(XcFuncId::MGGA_C_M06_L);
        }
        if lookup == "MGGA_C_M06_HF" {
            return Ok(XcFuncId::MGGA_C_M06_HF);
        }
        if lookup == "MGGA_C_M06" {
            return Ok(XcFuncId::MGGA_C_M06);
        }
        if lookup == "MGGA_C_M06_2X" {
            return Ok(XcFuncId::MGGA_C_M06_2X);
        }
        if lookup == "MGGA_C_M05" {
            return Ok(XcFuncId::MGGA_C_M05);
        }
        if lookup == "MGGA_C_M05_2X" {
            return Ok(XcFuncId::MGGA_C_M05_2X);
        }
        if lookup == "MGGA_C_PKZB" {
            return Ok(XcFuncId::MGGA_C_PKZB);
        }
        if lookup == "MGGA_C_BC95" {
            return Ok(XcFuncId::MGGA_C_BC95);
        }
        if lookup == "MGGA_C_REVTPSS" {
            return Ok(XcFuncId::MGGA_C_REVTPSS);
        }
        if lookup == "MGGA_XC_TPSSLYP1W" {
            return Ok(XcFuncId::MGGA_XC_TPSSLYP1W);
        }
        if lookup == "MGGA_X_MK00B" {
            return Ok(XcFuncId::MGGA_X_MK00B);
        }
        if lookup == "MGGA_X_BLOC" {
            return Ok(XcFuncId::MGGA_X_BLOC);
        }
        if lookup == "MGGA_X_MODTPSS" {
            return Ok(XcFuncId::MGGA_X_MODTPSS);
        }
        if lookup == "GGA_C_PBELOC" {
            return Ok(XcFuncId::GGA_C_PBELOC);
        }
        if lookup == "MGGA_C_TPSSLOC" {
            return Ok(XcFuncId::MGGA_C_TPSSLOC);
        }
        if lookup == "HYB_MGGA_X_MN12_SX" {
            return Ok(XcFuncId::HYB_MGGA_X_MN12_SX);
        }
        if lookup == "MGGA_X_MBEEF" {
            return Ok(XcFuncId::MGGA_X_MBEEF);
        }
        if lookup == "MGGA_X_MBEEFVDW" {
            return Ok(XcFuncId::MGGA_X_MBEEFVDW);
        }
        if lookup == "MGGA_C_TM" {
            return Ok(XcFuncId::MGGA_C_TM);
        }
        if lookup == "GGA_C_P86VWN" {
            return Ok(XcFuncId::GGA_C_P86VWN);
        }
        if lookup == "GGA_C_P86VWN_FT" {
            return Ok(XcFuncId::GGA_C_P86VWN_FT);
        }
        if lookup == "MGGA_XC_B97M_V" {
            return Ok(XcFuncId::MGGA_XC_B97M_V);
        }
        if lookup == "GGA_XC_VV10" {
            return Ok(XcFuncId::GGA_XC_VV10);
        }
        if lookup == "MGGA_X_JK" {
            return Ok(XcFuncId::MGGA_X_JK);
        }
        if lookup == "MGGA_X_MVS" {
            return Ok(XcFuncId::MGGA_X_MVS);
        }
        if lookup == "GGA_C_PBEFE" {
            return Ok(XcFuncId::GGA_C_PBEFE);
        }
        if lookup == "LDA_XC_KSDT" {
            return Ok(XcFuncId::LDA_XC_KSDT);
        }
        if lookup == "MGGA_X_MN15_L" {
            return Ok(XcFuncId::MGGA_X_MN15_L);
        }
        if lookup == "MGGA_C_MN15_L" {
            return Ok(XcFuncId::MGGA_C_MN15_L);
        }
        if lookup == "GGA_C_OP_PW91" {
            return Ok(XcFuncId::GGA_C_OP_PW91);
        }
        if lookup == "MGGA_X_SCAN" {
            return Ok(XcFuncId::MGGA_X_SCAN);
        }
        if lookup == "HYB_MGGA_X_SCAN0" {
            return Ok(XcFuncId::HYB_MGGA_X_SCAN0);
        }
        if lookup == "GGA_X_PBEFE" {
            return Ok(XcFuncId::GGA_X_PBEFE);
        }
        if lookup == "HYB_GGA_XC_B97_1P" {
            return Ok(XcFuncId::HYB_GGA_XC_B97_1P);
        }
        if lookup == "MGGA_C_SCAN" {
            return Ok(XcFuncId::MGGA_C_SCAN);
        }
        if lookup == "HYB_MGGA_X_MN15" {
            return Ok(XcFuncId::HYB_MGGA_X_MN15);
        }
        if lookup == "MGGA_C_MN15" {
            return Ok(XcFuncId::MGGA_C_MN15);
        }
        if lookup == "GGA_X_CAP" {
            return Ok(XcFuncId::GGA_X_CAP);
        }
        if lookup == "GGA_X_EB88" {
            return Ok(XcFuncId::GGA_X_EB88);
        }
        if lookup == "GGA_C_PBE_MOL" {
            return Ok(XcFuncId::GGA_C_PBE_MOL);
        }
        if lookup == "HYB_GGA_XC_PBE_MOL0" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE_MOL0);
        }
        if lookup == "HYB_GGA_XC_PBE_SOL0" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE_SOL0);
        }
        if lookup == "HYB_GGA_XC_PBEB0" {
            return Ok(XcFuncId::HYB_GGA_XC_PBEB0);
        }
        if lookup == "HYB_GGA_XC_PBE_MOLB0" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE_MOLB0);
        }
        if lookup == "GGA_K_ABSP3" {
            return Ok(XcFuncId::GGA_K_ABSP3);
        }
        if lookup == "GGA_K_ABSP4" {
            return Ok(XcFuncId::GGA_K_ABSP4);
        }
        if lookup == "HYB_MGGA_X_BMK" {
            return Ok(XcFuncId::HYB_MGGA_X_BMK);
        }
        if lookup == "GGA_C_BMK" {
            return Ok(XcFuncId::GGA_C_BMK);
        }
        if lookup == "GGA_C_TAU_HCTH" {
            return Ok(XcFuncId::GGA_C_TAU_HCTH);
        }
        if lookup == "HYB_MGGA_X_TAU_HCTH" {
            return Ok(XcFuncId::HYB_MGGA_X_TAU_HCTH);
        }
        if lookup == "GGA_C_HYB_TAU_HCTH" {
            return Ok(XcFuncId::GGA_C_HYB_TAU_HCTH);
        }
        if lookup == "MGGA_X_B00" {
            return Ok(XcFuncId::MGGA_X_B00);
        }
        if lookup == "GGA_X_BEEFVDW" {
            return Ok(XcFuncId::GGA_X_BEEFVDW);
        }
        if lookup == "GGA_XC_BEEFVDW" {
            return Ok(XcFuncId::GGA_XC_BEEFVDW);
        }
        if lookup == "LDA_C_CHACHIYO" {
            return Ok(XcFuncId::LDA_C_CHACHIYO);
        }
        if lookup == "MGGA_XC_HLE17" {
            return Ok(XcFuncId::MGGA_XC_HLE17);
        }
        if lookup == "LDA_C_LP96" {
            return Ok(XcFuncId::LDA_C_LP96);
        }
        if lookup == "HYB_GGA_XC_PBE50" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE50);
        }
        if lookup == "GGA_X_PBETRANS" {
            return Ok(XcFuncId::GGA_X_PBETRANS);
        }
        if lookup == "MGGA_C_SCAN_RVV10" {
            return Ok(XcFuncId::MGGA_C_SCAN_RVV10);
        }
        if lookup == "MGGA_X_REVM06_L" {
            return Ok(XcFuncId::MGGA_X_REVM06_L);
        }
        if lookup == "MGGA_C_REVM06_L" {
            return Ok(XcFuncId::MGGA_C_REVM06_L);
        }
        if lookup == "HYB_MGGA_X_M08_HX" {
            return Ok(XcFuncId::HYB_MGGA_X_M08_HX);
        }
        if lookup == "HYB_MGGA_X_M08_SO" {
            return Ok(XcFuncId::HYB_MGGA_X_M08_SO);
        }
        if lookup == "HYB_MGGA_X_M11" {
            return Ok(XcFuncId::HYB_MGGA_X_M11);
        }
        if lookup == "GGA_X_CHACHIYO" {
            return Ok(XcFuncId::GGA_X_CHACHIYO);
        }
        if lookup == "MGGA_X_RTPSS" {
            return Ok(XcFuncId::MGGA_X_RTPSS);
        }
        if lookup == "MGGA_X_MS2B" {
            return Ok(XcFuncId::MGGA_X_MS2B);
        }
        if lookup == "MGGA_X_MS2BS" {
            return Ok(XcFuncId::MGGA_X_MS2BS);
        }
        if lookup == "MGGA_X_MVSB" {
            return Ok(XcFuncId::MGGA_X_MVSB);
        }
        if lookup == "MGGA_X_MVSBS" {
            return Ok(XcFuncId::MGGA_X_MVSBS);
        }
        if lookup == "HYB_MGGA_X_REVM11" {
            return Ok(XcFuncId::HYB_MGGA_X_REVM11);
        }
        if lookup == "HYB_MGGA_X_REVM06" {
            return Ok(XcFuncId::HYB_MGGA_X_REVM06);
        }
        if lookup == "MGGA_C_REVM06" {
            return Ok(XcFuncId::MGGA_C_REVM06);
        }
        if lookup == "LDA_C_CHACHIYO_MOD" {
            return Ok(XcFuncId::LDA_C_CHACHIYO_MOD);
        }
        if lookup == "LDA_C_KARASIEV_MOD" {
            return Ok(XcFuncId::LDA_C_KARASIEV_MOD);
        }
        if lookup == "GGA_C_CHACHIYO" {
            return Ok(XcFuncId::GGA_C_CHACHIYO);
        }
        if lookup == "HYB_MGGA_X_M06_SX" {
            return Ok(XcFuncId::HYB_MGGA_X_M06_SX);
        }
        if lookup == "MGGA_C_M06_SX" {
            return Ok(XcFuncId::MGGA_C_M06_SX);
        }
        if lookup == "GGA_X_REVSSB_D" {
            return Ok(XcFuncId::GGA_X_REVSSB_D);
        }
        if lookup == "GGA_C_CCDF" {
            return Ok(XcFuncId::GGA_C_CCDF);
        }
        if lookup == "HYB_GGA_XC_HFLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_HFLYP);
        }
        if lookup == "HYB_GGA_XC_B3P86_NWCHEM" {
            return Ok(XcFuncId::HYB_GGA_XC_B3P86_NWCHEM);
        }
        if lookup == "GGA_X_PW91_MOD" {
            return Ok(XcFuncId::GGA_X_PW91_MOD);
        }
        if lookup == "LDA_C_W20" {
            return Ok(XcFuncId::LDA_C_W20);
        }
        if lookup == "LDA_XC_CORRKSDT" {
            return Ok(XcFuncId::LDA_XC_CORRKSDT);
        }
        if lookup == "MGGA_X_FT98" {
            return Ok(XcFuncId::MGGA_X_FT98);
        }
        if lookup == "GGA_X_PBE_MOD" {
            return Ok(XcFuncId::GGA_X_PBE_MOD);
        }
        if lookup == "GGA_X_PBE_GAUSSIAN" {
            return Ok(XcFuncId::GGA_X_PBE_GAUSSIAN);
        }
        if lookup == "GGA_C_PBE_GAUSSIAN" {
            return Ok(XcFuncId::GGA_C_PBE_GAUSSIAN);
        }
        if lookup == "MGGA_C_TPSS_GAUSSIAN" {
            return Ok(XcFuncId::MGGA_C_TPSS_GAUSSIAN);
        }
        if lookup == "GGA_X_NCAPR" {
            return Ok(XcFuncId::GGA_X_NCAPR);
        }
        if lookup == "HYB_GGA_XC_RELPBE0" {
            return Ok(XcFuncId::HYB_GGA_XC_RELPBE0);
        }
        if lookup == "MGGA_X_EEL" {
            return Ok(XcFuncId::MGGA_X_EEL);
        }
        if lookup == "GGA_XC_B97_3C" {
            return Ok(XcFuncId::GGA_XC_B97_3C);
        }
        if lookup == "LDA_C_EPC17" {
            return Ok(XcFuncId::LDA_C_EPC17);
        }
        if lookup == "LDA_C_EPC17_2" {
            return Ok(XcFuncId::LDA_C_EPC17_2);
        }
        if lookup == "LDA_C_EPC18_1" {
            return Ok(XcFuncId::LDA_C_EPC18_1);
        }
        if lookup == "LDA_C_EPC18_2" {
            return Ok(XcFuncId::LDA_C_EPC18_2);
        }
        if lookup == "GGA_XC_DLB97" {
            return Ok(XcFuncId::GGA_XC_DLB97);
        }
        if lookup == "MGGA_X_MSCAN" {
            return Ok(XcFuncId::MGGA_X_MSCAN);
        }
        if lookup == "MGGA_C_MSCAN" {
            return Ok(XcFuncId::MGGA_C_MSCAN);
        }
        if lookup == "GGA_X_T_PBE1" {
            return Ok(XcFuncId::GGA_X_T_PBE1);
        }
        if lookup == "GGA_X_T_PBE2" {
            return Ok(XcFuncId::GGA_X_T_PBE2);
        }
        if lookup == "LDA_X_T_SLOC" {
            return Ok(XcFuncId::LDA_X_T_SLOC);
        }
        if lookup == "GGA_X_BKL1" {
            return Ok(XcFuncId::GGA_X_BKL1);
        }
        if lookup == "GGA_X_BKL2" {
            return Ok(XcFuncId::GGA_X_BKL2);
        }
        if lookup == "HYB_MGGA_X_CF22D" {
            return Ok(XcFuncId::HYB_MGGA_X_CF22D);
        }
        if lookup == "MGGA_C_CF22D" {
            return Ok(XcFuncId::MGGA_C_CF22D);
        }
        if lookup == "MGGA_X_LAK" {
            return Ok(XcFuncId::MGGA_X_LAK);
        }
        if lookup == "GGA_C_BKL1" {
            return Ok(XcFuncId::GGA_C_BKL1);
        }
        if lookup == "GGA_C_BKL2" {
            return Ok(XcFuncId::GGA_C_BKL2);
        }
        if lookup == "MGGA_C_LAK" {
            return Ok(XcFuncId::MGGA_C_LAK);
        }
        if lookup == "GGA_X_DF3_OPT1" {
            return Ok(XcFuncId::GGA_X_DF3_OPT1);
        }
        if lookup == "GGA_X_DF3_OPT2" {
            return Ok(XcFuncId::GGA_X_DF3_OPT2);
        }
        if lookup == "HYB_GGA_XC_CQTP25" {
            return Ok(XcFuncId::HYB_GGA_XC_CQTP25);
        }
        if lookup == "HYB_GGA_XC_OPB3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_OPB3LYP);
        }
        if lookup == "MGGA_C_CC" {
            return Ok(XcFuncId::MGGA_C_CC);
        }
        if lookup == "MGGA_C_CCALDA" {
            return Ok(XcFuncId::MGGA_C_CCALDA);
        }
        if lookup == "HYB_MGGA_XC_BR3P86" {
            return Ok(XcFuncId::HYB_MGGA_XC_BR3P86);
        }
        if lookup == "HYB_GGA_XC_CASE21" {
            return Ok(XcFuncId::HYB_GGA_XC_CASE21);
        }
        if lookup == "MGGA_C_RREGTM" {
            return Ok(XcFuncId::MGGA_C_RREGTM);
        }
        if lookup == "HYB_GGA_XC_PBE_2X" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE_2X);
        }
        if lookup == "HYB_GGA_XC_PBE38" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE38);
        }
        if lookup == "HYB_GGA_XC_B3LYP3" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYP3);
        }
        if lookup == "HYB_GGA_XC_CAM_O3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_O3LYP);
        }
        if lookup == "HYB_MGGA_XC_TPSS0" {
            return Ok(XcFuncId::HYB_MGGA_XC_TPSS0);
        }
        if lookup == "MGGA_C_B94" {
            return Ok(XcFuncId::MGGA_C_B94);
        }
        if lookup == "HYB_MGGA_XC_B94_HYB" {
            return Ok(XcFuncId::HYB_MGGA_XC_B94_HYB);
        }
        if lookup == "HYB_GGA_XC_WB97X_D3" {
            return Ok(XcFuncId::HYB_GGA_XC_WB97X_D3);
        }
        if lookup == "HYB_GGA_XC_LC_BLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_BLYP);
        }
        if lookup == "HYB_GGA_XC_B3PW91" {
            return Ok(XcFuncId::HYB_GGA_XC_B3PW91);
        }
        if lookup == "HYB_GGA_XC_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYP);
        }
        if lookup == "HYB_GGA_XC_B3P86" {
            return Ok(XcFuncId::HYB_GGA_XC_B3P86);
        }
        if lookup == "HYB_GGA_XC_O3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_O3LYP);
        }
        if lookup == "HYB_GGA_XC_MPW1K" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW1K);
        }
        if lookup == "HYB_GGA_XC_PBEH" {
            return Ok(XcFuncId::HYB_GGA_XC_PBEH);
        }
        if lookup == "HYB_GGA_XC_B97" {
            return Ok(XcFuncId::HYB_GGA_XC_B97);
        }
        if lookup == "HYB_GGA_XC_B97_1" {
            return Ok(XcFuncId::HYB_GGA_XC_B97_1);
        }
        if lookup == "HYB_GGA_XC_APF" {
            return Ok(XcFuncId::HYB_GGA_XC_APF);
        }
        if lookup == "HYB_GGA_XC_B97_2" {
            return Ok(XcFuncId::HYB_GGA_XC_B97_2);
        }
        if lookup == "HYB_GGA_XC_X3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_X3LYP);
        }
        if lookup == "HYB_GGA_XC_B1WC" {
            return Ok(XcFuncId::HYB_GGA_XC_B1WC);
        }
        if lookup == "HYB_GGA_XC_B97_K" {
            return Ok(XcFuncId::HYB_GGA_XC_B97_K);
        }
        if lookup == "HYB_GGA_XC_B97_3" {
            return Ok(XcFuncId::HYB_GGA_XC_B97_3);
        }
        if lookup == "HYB_GGA_XC_MPW3PW" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW3PW);
        }
        if lookup == "HYB_GGA_XC_B1LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_B1LYP);
        }
        if lookup == "HYB_GGA_XC_B1PW91" {
            return Ok(XcFuncId::HYB_GGA_XC_B1PW91);
        }
        if lookup == "HYB_GGA_XC_MPW1PW" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW1PW);
        }
        if lookup == "HYB_GGA_XC_MPW3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW3LYP);
        }
        if lookup == "HYB_GGA_XC_SB98_1A" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_1A);
        }
        if lookup == "HYB_GGA_XC_SB98_1B" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_1B);
        }
        if lookup == "HYB_GGA_XC_SB98_1C" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_1C);
        }
        if lookup == "HYB_GGA_XC_SB98_2A" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_2A);
        }
        if lookup == "HYB_GGA_XC_SB98_2B" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_2B);
        }
        if lookup == "HYB_GGA_XC_SB98_2C" {
            return Ok(XcFuncId::HYB_GGA_XC_SB98_2C);
        }
        if lookup == "HYB_GGA_X_SOGGA11_X" {
            return Ok(XcFuncId::HYB_GGA_X_SOGGA11_X);
        }
        if lookup == "HYB_GGA_XC_HSE03" {
            return Ok(XcFuncId::HYB_GGA_XC_HSE03);
        }
        if lookup == "HYB_GGA_XC_HSE06" {
            return Ok(XcFuncId::HYB_GGA_XC_HSE06);
        }
        if lookup == "HYB_GGA_XC_HJS_PBE" {
            return Ok(XcFuncId::HYB_GGA_XC_HJS_PBE);
        }
        if lookup == "HYB_GGA_XC_HJS_PBE_SOL" {
            return Ok(XcFuncId::HYB_GGA_XC_HJS_PBE_SOL);
        }
        if lookup == "HYB_GGA_XC_HJS_B88" {
            return Ok(XcFuncId::HYB_GGA_XC_HJS_B88);
        }
        if lookup == "HYB_GGA_XC_HJS_B97X" {
            return Ok(XcFuncId::HYB_GGA_XC_HJS_B97X);
        }
        if lookup == "HYB_GGA_XC_CAM_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_B3LYP);
        }
        if lookup == "HYB_GGA_XC_TUNED_CAM_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_TUNED_CAM_B3LYP);
        }
        if lookup == "HYB_GGA_XC_BHANDH" {
            return Ok(XcFuncId::HYB_GGA_XC_BHANDH);
        }
        if lookup == "HYB_GGA_XC_BHANDHLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_BHANDHLYP);
        }
        if lookup == "HYB_GGA_XC_MB3LYP_RC04" {
            return Ok(XcFuncId::HYB_GGA_XC_MB3LYP_RC04);
        }
        if lookup == "HYB_MGGA_X_M05" {
            return Ok(XcFuncId::HYB_MGGA_X_M05);
        }
        if lookup == "HYB_MGGA_X_M05_2X" {
            return Ok(XcFuncId::HYB_MGGA_X_M05_2X);
        }
        if lookup == "HYB_MGGA_XC_B88B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_B88B95);
        }
        if lookup == "HYB_MGGA_XC_B86B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_B86B95);
        }
        if lookup == "HYB_MGGA_XC_PW86B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_PW86B95);
        }
        if lookup == "HYB_MGGA_XC_BB1K" {
            return Ok(XcFuncId::HYB_MGGA_XC_BB1K);
        }
        if lookup == "HYB_MGGA_X_M06_HF" {
            return Ok(XcFuncId::HYB_MGGA_X_M06_HF);
        }
        if lookup == "HYB_MGGA_XC_MPW1B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_MPW1B95);
        }
        if lookup == "HYB_MGGA_XC_MPWB1K" {
            return Ok(XcFuncId::HYB_MGGA_XC_MPWB1K);
        }
        if lookup == "HYB_MGGA_XC_X1B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_X1B95);
        }
        if lookup == "HYB_MGGA_XC_XB1K" {
            return Ok(XcFuncId::HYB_MGGA_XC_XB1K);
        }
        if lookup == "HYB_MGGA_X_M06" {
            return Ok(XcFuncId::HYB_MGGA_X_M06);
        }
        if lookup == "HYB_MGGA_X_M06_2X" {
            return Ok(XcFuncId::HYB_MGGA_X_M06_2X);
        }
        if lookup == "HYB_MGGA_XC_PW6B95" {
            return Ok(XcFuncId::HYB_MGGA_XC_PW6B95);
        }
        if lookup == "HYB_MGGA_XC_PWB6K" {
            return Ok(XcFuncId::HYB_MGGA_XC_PWB6K);
        }
        if lookup == "HYB_GGA_XC_MPWLYP1M" {
            return Ok(XcFuncId::HYB_GGA_XC_MPWLYP1M);
        }
        if lookup == "HYB_GGA_XC_REVB3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_REVB3LYP);
        }
        if lookup == "HYB_GGA_XC_CAMY_BLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_CAMY_BLYP);
        }
        if lookup == "HYB_GGA_XC_PBE0_13" {
            return Ok(XcFuncId::HYB_GGA_XC_PBE0_13);
        }
        if lookup == "HYB_MGGA_XC_TPSSH" {
            return Ok(XcFuncId::HYB_MGGA_XC_TPSSH);
        }
        if lookup == "HYB_MGGA_XC_REVTPSSH" {
            return Ok(XcFuncId::HYB_MGGA_XC_REVTPSSH);
        }
        if lookup == "HYB_GGA_XC_B3LYPS" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYPS);
        }
        if lookup == "HYB_GGA_XC_QTP17" {
            return Ok(XcFuncId::HYB_GGA_XC_QTP17);
        }
        if lookup == "HYB_GGA_XC_B3LYP_MCM1" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYP_MCM1);
        }
        if lookup == "HYB_GGA_XC_B3LYP_MCM2" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYP_MCM2);
        }
        if lookup == "HYB_GGA_XC_WB97" {
            return Ok(XcFuncId::HYB_GGA_XC_WB97);
        }
        if lookup == "HYB_GGA_XC_WB97X" {
            return Ok(XcFuncId::HYB_GGA_XC_WB97X);
        }
        if lookup == "HYB_GGA_XC_LRC_WPBEH" {
            return Ok(XcFuncId::HYB_GGA_XC_LRC_WPBEH);
        }
        if lookup == "HYB_GGA_XC_WB97X_V" {
            return Ok(XcFuncId::HYB_GGA_XC_WB97X_V);
        }
        if lookup == "HYB_GGA_XC_LCY_PBE" {
            return Ok(XcFuncId::HYB_GGA_XC_LCY_PBE);
        }
        if lookup == "HYB_GGA_XC_LCY_BLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_LCY_BLYP);
        }
        if lookup == "HYB_GGA_XC_LC_VV10" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_VV10);
        }
        if lookup == "HYB_GGA_XC_CAMY_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_CAMY_B3LYP);
        }
        if lookup == "HYB_GGA_XC_WB97X_D" {
            return Ok(XcFuncId::HYB_GGA_XC_WB97X_D);
        }
        if lookup == "HYB_GGA_XC_HPBEINT" {
            return Ok(XcFuncId::HYB_GGA_XC_HPBEINT);
        }
        if lookup == "HYB_GGA_XC_LRC_WPBE" {
            return Ok(XcFuncId::HYB_GGA_XC_LRC_WPBE);
        }
        if lookup == "HYB_MGGA_X_MVSH" {
            return Ok(XcFuncId::HYB_MGGA_X_MVSH);
        }
        if lookup == "HYB_GGA_XC_B3LYP5" {
            return Ok(XcFuncId::HYB_GGA_XC_B3LYP5);
        }
        if lookup == "HYB_GGA_XC_EDF2" {
            return Ok(XcFuncId::HYB_GGA_XC_EDF2);
        }
        if lookup == "HYB_GGA_XC_CAP0" {
            return Ok(XcFuncId::HYB_GGA_XC_CAP0);
        }
        if lookup == "HYB_GGA_XC_LC_WPBE" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_WPBE);
        }
        if lookup == "HYB_GGA_XC_HSE12" {
            return Ok(XcFuncId::HYB_GGA_XC_HSE12);
        }
        if lookup == "HYB_GGA_XC_HSE12S" {
            return Ok(XcFuncId::HYB_GGA_XC_HSE12S);
        }
        if lookup == "HYB_GGA_XC_HSE_SOL" {
            return Ok(XcFuncId::HYB_GGA_XC_HSE_SOL);
        }
        if lookup == "HYB_GGA_XC_CAM_QTP_01" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_01);
        }
        if lookup == "HYB_GGA_XC_MPW1LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW1LYP);
        }
        if lookup == "HYB_GGA_XC_MPW1PBE" {
            return Ok(XcFuncId::HYB_GGA_XC_MPW1PBE);
        }
        if lookup == "HYB_GGA_XC_KMLYP" {
            return Ok(XcFuncId::HYB_GGA_XC_KMLYP);
        }
        if lookup == "HYB_GGA_XC_LC_WPBE_WHS" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_WPBE_WHS);
        }
        if lookup == "HYB_GGA_XC_LC_WPBEH_WHS" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_WPBEH_WHS);
        }
        if lookup == "HYB_GGA_XC_LC_WPBE08_WHS" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_WPBE08_WHS);
        }
        if lookup == "HYB_GGA_XC_LC_WPBESOL_WHS" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_WPBESOL_WHS);
        }
        if lookup == "HYB_GGA_XC_CAM_QTP_00" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_00);
        }
        if lookup == "HYB_GGA_XC_CAM_QTP_02" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_02);
        }
        if lookup == "HYB_GGA_XC_LC_QTP" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_QTP);
        }
        if lookup == "MGGA_X_RSCAN" {
            return Ok(XcFuncId::MGGA_X_RSCAN);
        }
        if lookup == "MGGA_C_RSCAN" {
            return Ok(XcFuncId::MGGA_C_RSCAN);
        }
        if lookup == "GGA_X_S12G" {
            return Ok(XcFuncId::GGA_X_S12G);
        }
        if lookup == "HYB_GGA_X_S12H" {
            return Ok(XcFuncId::HYB_GGA_X_S12H);
        }
        if lookup == "MGGA_X_R2SCAN" {
            return Ok(XcFuncId::MGGA_X_R2SCAN);
        }
        if lookup == "MGGA_C_R2SCAN" {
            return Ok(XcFuncId::MGGA_C_R2SCAN);
        }
        if lookup == "HYB_GGA_XC_BLYP35" {
            return Ok(XcFuncId::HYB_GGA_XC_BLYP35);
        }
        if lookup == "GGA_K_VW" {
            return Ok(XcFuncId::GGA_K_VW);
        }
        if lookup == "GGA_K_GE2" {
            return Ok(XcFuncId::GGA_K_GE2);
        }
        if lookup == "GGA_K_GOLDEN" {
            return Ok(XcFuncId::GGA_K_GOLDEN);
        }
        if lookup == "GGA_K_YT65" {
            return Ok(XcFuncId::GGA_K_YT65);
        }
        if lookup == "GGA_K_BALTIN" {
            return Ok(XcFuncId::GGA_K_BALTIN);
        }
        if lookup == "GGA_K_LIEB" {
            return Ok(XcFuncId::GGA_K_LIEB);
        }
        if lookup == "GGA_K_ABSP1" {
            return Ok(XcFuncId::GGA_K_ABSP1);
        }
        if lookup == "GGA_K_ABSP2" {
            return Ok(XcFuncId::GGA_K_ABSP2);
        }
        if lookup == "GGA_K_GR" {
            return Ok(XcFuncId::GGA_K_GR);
        }
        if lookup == "GGA_K_LUDENA" {
            return Ok(XcFuncId::GGA_K_LUDENA);
        }
        if lookup == "GGA_K_GP85" {
            return Ok(XcFuncId::GGA_K_GP85);
        }
        if lookup == "GGA_K_PEARSON" {
            return Ok(XcFuncId::GGA_K_PEARSON);
        }
        if lookup == "GGA_K_OL1" {
            return Ok(XcFuncId::GGA_K_OL1);
        }
        if lookup == "GGA_K_OL2" {
            return Ok(XcFuncId::GGA_K_OL2);
        }
        if lookup == "GGA_K_FR_B88" {
            return Ok(XcFuncId::GGA_K_FR_B88);
        }
        if lookup == "GGA_K_FR_PW86" {
            return Ok(XcFuncId::GGA_K_FR_PW86);
        }
        if lookup == "GGA_K_DK" {
            return Ok(XcFuncId::GGA_K_DK);
        }
        if lookup == "GGA_K_PERDEW" {
            return Ok(XcFuncId::GGA_K_PERDEW);
        }
        if lookup == "GGA_K_VSK" {
            return Ok(XcFuncId::GGA_K_VSK);
        }
        if lookup == "GGA_K_VJKS" {
            return Ok(XcFuncId::GGA_K_VJKS);
        }
        if lookup == "GGA_K_ERNZERHOF" {
            return Ok(XcFuncId::GGA_K_ERNZERHOF);
        }
        if lookup == "GGA_K_LC94" {
            return Ok(XcFuncId::GGA_K_LC94);
        }
        if lookup == "GGA_K_LLP" {
            return Ok(XcFuncId::GGA_K_LLP);
        }
        if lookup == "GGA_K_THAKKAR" {
            return Ok(XcFuncId::GGA_K_THAKKAR);
        }
        if lookup == "GGA_X_WPBEH" {
            return Ok(XcFuncId::GGA_X_WPBEH);
        }
        if lookup == "GGA_X_HJS_PBE" {
            return Ok(XcFuncId::GGA_X_HJS_PBE);
        }
        if lookup == "GGA_X_HJS_PBE_SOL" {
            return Ok(XcFuncId::GGA_X_HJS_PBE_SOL);
        }
        if lookup == "GGA_X_HJS_B88" {
            return Ok(XcFuncId::GGA_X_HJS_B88);
        }
        if lookup == "GGA_X_HJS_B97X" {
            return Ok(XcFuncId::GGA_X_HJS_B97X);
        }
        if lookup == "GGA_X_ITYH" {
            return Ok(XcFuncId::GGA_X_ITYH);
        }
        if lookup == "GGA_X_SFAT" {
            return Ok(XcFuncId::GGA_X_SFAT);
        }
        if lookup == "HYB_MGGA_XC_WB97M_V" {
            return Ok(XcFuncId::HYB_MGGA_XC_WB97M_V);
        }
        if lookup == "LDA_X_REL" {
            return Ok(XcFuncId::LDA_X_REL);
        }
        if lookup == "GGA_X_SG4" {
            return Ok(XcFuncId::GGA_X_SG4);
        }
        if lookup == "GGA_C_SG4" {
            return Ok(XcFuncId::GGA_C_SG4);
        }
        if lookup == "GGA_X_GG99" {
            return Ok(XcFuncId::GGA_X_GG99);
        }
        if lookup == "LDA_XC_1D_EHWLRG_1" {
            return Ok(XcFuncId::LDA_XC_1D_EHWLRG_1);
        }
        if lookup == "LDA_XC_1D_EHWLRG_2" {
            return Ok(XcFuncId::LDA_XC_1D_EHWLRG_2);
        }
        if lookup == "LDA_XC_1D_EHWLRG_3" {
            return Ok(XcFuncId::LDA_XC_1D_EHWLRG_3);
        }
        if lookup == "GGA_X_PBEPOW" {
            return Ok(XcFuncId::GGA_X_PBEPOW);
        }
        if lookup == "MGGA_X_TM" {
            return Ok(XcFuncId::MGGA_X_TM);
        }
        if lookup == "MGGA_X_VT84" {
            return Ok(XcFuncId::MGGA_X_VT84);
        }
        if lookup == "MGGA_X_SA_TPSS" {
            return Ok(XcFuncId::MGGA_X_SA_TPSS);
        }
        if lookup == "MGGA_K_PC07" {
            return Ok(XcFuncId::MGGA_K_PC07);
        }
        if lookup == "GGA_X_KGG99" {
            return Ok(XcFuncId::GGA_X_KGG99);
        }
        if lookup == "GGA_XC_HLE16" {
            return Ok(XcFuncId::GGA_XC_HLE16);
        }
        if lookup == "LDA_X_ERF" {
            return Ok(XcFuncId::LDA_X_ERF);
        }
        if lookup == "LDA_XC_LP_A" {
            return Ok(XcFuncId::LDA_XC_LP_A);
        }
        if lookup == "LDA_XC_LP_B" {
            return Ok(XcFuncId::LDA_XC_LP_B);
        }
        if lookup == "LDA_X_RAE" {
            return Ok(XcFuncId::LDA_X_RAE);
        }
        if lookup == "LDA_K_ZLP" {
            return Ok(XcFuncId::LDA_K_ZLP);
        }
        if lookup == "LDA_C_MCWEENY" {
            return Ok(XcFuncId::LDA_C_MCWEENY);
        }
        if lookup == "LDA_C_BR78" {
            return Ok(XcFuncId::LDA_C_BR78);
        }
        if lookup == "GGA_C_SCAN_E0" {
            return Ok(XcFuncId::GGA_C_SCAN_E0);
        }
        if lookup == "LDA_C_PK09" {
            return Ok(XcFuncId::LDA_C_PK09);
        }
        if lookup == "GGA_C_GAPC" {
            return Ok(XcFuncId::GGA_C_GAPC);
        }
        if lookup == "GGA_C_GAPLOC" {
            return Ok(XcFuncId::GGA_C_GAPLOC);
        }
        if lookup == "GGA_C_ZVPBEINT" {
            return Ok(XcFuncId::GGA_C_ZVPBEINT);
        }
        if lookup == "GGA_C_ZVPBESOL" {
            return Ok(XcFuncId::GGA_C_ZVPBESOL);
        }
        if lookup == "GGA_C_TM_LYP" {
            return Ok(XcFuncId::GGA_C_TM_LYP);
        }
        if lookup == "GGA_C_TM_PBE" {
            return Ok(XcFuncId::GGA_C_TM_PBE);
        }
        if lookup == "GGA_C_W94" {
            return Ok(XcFuncId::GGA_C_W94);
        }
        if lookup == "MGGA_C_KCIS" {
            return Ok(XcFuncId::MGGA_C_KCIS);
        }
        if lookup == "HYB_MGGA_XC_B0KCIS" {
            return Ok(XcFuncId::HYB_MGGA_XC_B0KCIS);
        }
        if lookup == "MGGA_XC_LP90" {
            return Ok(XcFuncId::MGGA_XC_LP90);
        }
        if lookup == "GGA_C_CS1" {
            return Ok(XcFuncId::GGA_C_CS1);
        }
        if lookup == "HYB_MGGA_XC_MPW1KCIS" {
            return Ok(XcFuncId::HYB_MGGA_XC_MPW1KCIS);
        }
        if lookup == "HYB_MGGA_XC_MPWKCIS1K" {
            return Ok(XcFuncId::HYB_MGGA_XC_MPWKCIS1K);
        }
        if lookup == "HYB_MGGA_XC_PBE1KCIS" {
            return Ok(XcFuncId::HYB_MGGA_XC_PBE1KCIS);
        }
        if lookup == "HYB_MGGA_XC_TPSS1KCIS" {
            return Ok(XcFuncId::HYB_MGGA_XC_TPSS1KCIS);
        }
        if lookup == "GGA_X_B88M" {
            return Ok(XcFuncId::GGA_X_B88M);
        }
        if lookup == "MGGA_C_B88" {
            return Ok(XcFuncId::MGGA_C_B88);
        }
        if lookup == "HYB_GGA_XC_B5050LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_B5050LYP);
        }
        if lookup == "LDA_C_OW_LYP" {
            return Ok(XcFuncId::LDA_C_OW_LYP);
        }
        if lookup == "LDA_C_OW" {
            return Ok(XcFuncId::LDA_C_OW);
        }
        if lookup == "MGGA_X_GX" {
            return Ok(XcFuncId::MGGA_X_GX);
        }
        if lookup == "MGGA_X_PBE_GX" {
            return Ok(XcFuncId::MGGA_X_PBE_GX);
        }
        if lookup == "LDA_XC_GDSMFB" {
            return Ok(XcFuncId::LDA_XC_GDSMFB);
        }
        if lookup == "LDA_C_GK72" {
            return Ok(XcFuncId::LDA_C_GK72);
        }
        if lookup == "LDA_C_KARASIEV" {
            return Ok(XcFuncId::LDA_C_KARASIEV);
        }
        if lookup == "LDA_K_LP96" {
            return Ok(XcFuncId::LDA_K_LP96);
        }
        if lookup == "MGGA_X_REVSCAN" {
            return Ok(XcFuncId::MGGA_X_REVSCAN);
        }
        if lookup == "MGGA_C_REVSCAN" {
            return Ok(XcFuncId::MGGA_C_REVSCAN);
        }
        if lookup == "HYB_MGGA_X_REVSCAN0" {
            return Ok(XcFuncId::HYB_MGGA_X_REVSCAN0);
        }
        if lookup == "MGGA_C_SCAN_VV10" {
            return Ok(XcFuncId::MGGA_C_SCAN_VV10);
        }
        if lookup == "MGGA_C_REVSCAN_VV10" {
            return Ok(XcFuncId::MGGA_C_REVSCAN_VV10);
        }
        if lookup == "MGGA_X_BR89_EXPLICIT" {
            return Ok(XcFuncId::MGGA_X_BR89_EXPLICIT);
        }
        if lookup == "GGA_XC_KT3" {
            return Ok(XcFuncId::GGA_XC_KT3);
        }
        if lookup == "HYB_LDA_XC_BN05" {
            return Ok(XcFuncId::HYB_LDA_XC_BN05);
        }
        if lookup == "HYB_GGA_XC_LB07" {
            return Ok(XcFuncId::HYB_GGA_XC_LB07);
        }
        if lookup == "LDA_C_PMGB06" {
            return Ok(XcFuncId::LDA_C_PMGB06);
        }
        if lookup == "GGA_K_GDS08" {
            return Ok(XcFuncId::GGA_K_GDS08);
        }
        if lookup == "GGA_K_GHDS10" {
            return Ok(XcFuncId::GGA_K_GHDS10);
        }
        if lookup == "GGA_K_GHDS10R" {
            return Ok(XcFuncId::GGA_K_GHDS10R);
        }
        if lookup == "GGA_K_TKVLN" {
            return Ok(XcFuncId::GGA_K_TKVLN);
        }
        if lookup == "GGA_K_PBE3" {
            return Ok(XcFuncId::GGA_K_PBE3);
        }
        if lookup == "GGA_K_PBE4" {
            return Ok(XcFuncId::GGA_K_PBE4);
        }
        if lookup == "GGA_K_EXP4" {
            return Ok(XcFuncId::GGA_K_EXP4);
        }
        if lookup == "HYB_MGGA_XC_B98" {
            return Ok(XcFuncId::HYB_MGGA_XC_B98);
        }
        if lookup == "LDA_XC_TIH" {
            return Ok(XcFuncId::LDA_XC_TIH);
        }
        if lookup == "LDA_X_1D_EXPONENTIAL" {
            return Ok(XcFuncId::LDA_X_1D_EXPONENTIAL);
        }
        if lookup == "GGA_X_SFAT_PBE" {
            return Ok(XcFuncId::GGA_X_SFAT_PBE);
        }
        if lookup == "MGGA_X_BR89_EXPLICIT_1" {
            return Ok(XcFuncId::MGGA_X_BR89_EXPLICIT_1);
        }
        if lookup == "MGGA_X_REGTPSS" {
            return Ok(XcFuncId::MGGA_X_REGTPSS);
        }
        if lookup == "GGA_X_FD_LB94" {
            return Ok(XcFuncId::GGA_X_FD_LB94);
        }
        if lookup == "GGA_X_FD_REVLB94" {
            return Ok(XcFuncId::GGA_X_FD_REVLB94);
        }
        if lookup == "GGA_C_ZVPBELOC" {
            return Ok(XcFuncId::GGA_C_ZVPBELOC);
        }
        if lookup == "HYB_GGA_XC_APBE0" {
            return Ok(XcFuncId::HYB_GGA_XC_APBE0);
        }
        if lookup == "HYB_GGA_XC_HAPBE" {
            return Ok(XcFuncId::HYB_GGA_XC_HAPBE);
        }
        if lookup == "MGGA_X_2D_JS17" {
            return Ok(XcFuncId::MGGA_X_2D_JS17);
        }
        if lookup == "HYB_GGA_XC_RCAM_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_RCAM_B3LYP);
        }
        if lookup == "HYB_GGA_XC_WC04" {
            return Ok(XcFuncId::HYB_GGA_XC_WC04);
        }
        if lookup == "HYB_GGA_XC_WP04" {
            return Ok(XcFuncId::HYB_GGA_XC_WP04);
        }
        if lookup == "GGA_K_LKT" {
            return Ok(XcFuncId::GGA_K_LKT);
        }
        if lookup == "HYB_GGA_XC_CAMH_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_CAMH_B3LYP);
        }
        if lookup == "HYB_GGA_XC_WHPBE0" {
            return Ok(XcFuncId::HYB_GGA_XC_WHPBE0);
        }
        if lookup == "GGA_K_PBE2" {
            return Ok(XcFuncId::GGA_K_PBE2);
        }
        if lookup == "MGGA_K_L04" {
            return Ok(XcFuncId::MGGA_K_L04);
        }
        if lookup == "MGGA_K_L06" {
            return Ok(XcFuncId::MGGA_K_L06);
        }
        if lookup == "GGA_K_VT84F" {
            return Ok(XcFuncId::GGA_K_VT84F);
        }
        if lookup == "GGA_K_LGAP" {
            return Ok(XcFuncId::GGA_K_LGAP);
        }
        if lookup == "MGGA_K_RDA" {
            return Ok(XcFuncId::MGGA_K_RDA);
        }
        if lookup == "GGA_X_ITYH_OPTX" {
            return Ok(XcFuncId::GGA_X_ITYH_OPTX);
        }
        if lookup == "GGA_X_ITYH_PBE" {
            return Ok(XcFuncId::GGA_X_ITYH_PBE);
        }
        if lookup == "GGA_C_LYPR" {
            return Ok(XcFuncId::GGA_C_LYPR);
        }
        if lookup == "HYB_GGA_XC_LC_BLYP_EA" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_BLYP_EA);
        }
        if lookup == "MGGA_X_REGTM" {
            return Ok(XcFuncId::MGGA_X_REGTM);
        }
        if lookup == "MGGA_K_GEA2" {
            return Ok(XcFuncId::MGGA_K_GEA2);
        }
        if lookup == "MGGA_K_GEA4" {
            return Ok(XcFuncId::MGGA_K_GEA4);
        }
        if lookup == "MGGA_K_CSK1" {
            return Ok(XcFuncId::MGGA_K_CSK1);
        }
        if lookup == "MGGA_K_CSK4" {
            return Ok(XcFuncId::MGGA_K_CSK4);
        }
        if lookup == "MGGA_K_CSK_LOC1" {
            return Ok(XcFuncId::MGGA_K_CSK_LOC1);
        }
        if lookup == "MGGA_K_CSK_LOC4" {
            return Ok(XcFuncId::MGGA_K_CSK_LOC4);
        }
        if lookup == "GGA_K_LGAP_GE" {
            return Ok(XcFuncId::GGA_K_LGAP_GE);
        }
        if lookup == "MGGA_K_PC07_OPT" {
            return Ok(XcFuncId::MGGA_K_PC07_OPT);
        }
        if lookup == "GGA_K_TFVW_OPT" {
            return Ok(XcFuncId::GGA_K_TFVW_OPT);
        }
        if lookup == "HYB_GGA_XC_LC_BOP" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_BOP);
        }
        if lookup == "HYB_GGA_XC_LC_PBEOP" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_PBEOP);
        }
        if lookup == "MGGA_C_KCISK" {
            return Ok(XcFuncId::MGGA_C_KCISK);
        }
        if lookup == "HYB_GGA_XC_LC_BLYPR" {
            return Ok(XcFuncId::HYB_GGA_XC_LC_BLYPR);
        }
        if lookup == "HYB_GGA_XC_MCAM_B3LYP" {
            return Ok(XcFuncId::HYB_GGA_XC_MCAM_B3LYP);
        }
        if lookup == "LDA_X_YUKAWA" {
            return Ok(XcFuncId::LDA_X_YUKAWA);
        }
        if lookup == "MGGA_C_R2SCAN01" {
            return Ok(XcFuncId::MGGA_C_R2SCAN01);
        }
        if lookup == "MGGA_C_RMGGAC" {
            return Ok(XcFuncId::MGGA_C_RMGGAC);
        }
        if lookup == "MGGA_X_MCML" {
            return Ok(XcFuncId::MGGA_X_MCML);
        }
        if lookup == "MGGA_X_R2SCAN01" {
            return Ok(XcFuncId::MGGA_X_R2SCAN01);
        }
        if lookup == "HYB_GGA_X_CAM_S12G" {
            return Ok(XcFuncId::HYB_GGA_X_CAM_S12G);
        }
        if lookup == "HYB_GGA_X_CAM_S12H" {
            return Ok(XcFuncId::HYB_GGA_X_CAM_S12H);
        }
        if lookup == "MGGA_X_RPPSCAN" {
            return Ok(XcFuncId::MGGA_X_RPPSCAN);
        }
        if lookup == "MGGA_C_RPPSCAN" {
            return Ok(XcFuncId::MGGA_C_RPPSCAN);
        }
        if lookup == "MGGA_X_R4SCAN" {
            return Ok(XcFuncId::MGGA_X_R4SCAN);
        }
        if lookup == "MGGA_X_VCML" {
            return Ok(XcFuncId::MGGA_X_VCML);
        }
        if lookup == "MGGA_XC_VCML_RVV10" {
            return Ok(XcFuncId::MGGA_XC_VCML_RVV10);
        }
        if lookup == "HYB_LDA_X_ERF" {
            return Ok(XcFuncId::HYB_LDA_X_ERF);
        }
        if lookup == "LDA_C_PW_ERF" {
            return Ok(XcFuncId::LDA_C_PW_ERF);
        }
        if lookup == "GGA_X_PBE_ERF_GWS" {
            return Ok(XcFuncId::GGA_X_PBE_ERF_GWS);
        }
        if lookup == "HYB_GGA_X_PBE_ERF_GWS" {
            return Ok(XcFuncId::HYB_GGA_X_PBE_ERF_GWS);
        }
        if lookup == "GGA_C_PBE_ERF_GWS" {
            return Ok(XcFuncId::GGA_C_PBE_ERF_GWS);
        }
        if lookup == "HYB_MGGA_XC_GAS22" {
            return Ok(XcFuncId::HYB_MGGA_XC_GAS22);
        }
        if lookup == "HYB_MGGA_XC_R2SCANH" {
            return Ok(XcFuncId::HYB_MGGA_XC_R2SCANH);
        }
        if lookup == "HYB_MGGA_XC_R2SCAN0" {
            return Ok(XcFuncId::HYB_MGGA_XC_R2SCAN0);
        }
        if lookup == "HYB_MGGA_XC_R2SCAN50" {
            return Ok(XcFuncId::HYB_MGGA_XC_R2SCAN50);
        }
        if lookup == "HYB_MGGA_X_WR2SCAN" {
            return Ok(XcFuncId::HYB_MGGA_X_WR2SCAN);
        }
        if lookup == "HYB_GGA_XC_CAM_PBEH" {
            return Ok(XcFuncId::HYB_GGA_XC_CAM_PBEH);
        }
        if lookup == "HYB_GGA_XC_CAMY_PBEH" {
            return Ok(XcFuncId::HYB_GGA_XC_CAMY_PBEH);
        }
        if lookup == "LDA_C_UPW92" {
            return Ok(XcFuncId::LDA_C_UPW92);
        }
        if lookup == "LDA_C_RPW92" {
            return Ok(XcFuncId::LDA_C_RPW92);
        }
        if lookup == "MGGA_X_TLDA" {
            return Ok(XcFuncId::MGGA_X_TLDA);
        }
        if lookup == "MGGA_X_EDMGGA" {
            return Ok(XcFuncId::MGGA_X_EDMGGA);
        }
        if lookup == "MGGA_X_GDME_NV" {
            return Ok(XcFuncId::MGGA_X_GDME_NV);
        }
        if lookup == "MGGA_X_RLDA" {
            return Ok(XcFuncId::MGGA_X_RLDA);
        }
        if lookup == "MGGA_X_GDME_0" {
            return Ok(XcFuncId::MGGA_X_GDME_0);
        }
        if lookup == "MGGA_X_GDME_KOS" {
            return Ok(XcFuncId::MGGA_X_GDME_KOS);
        }
        if lookup == "MGGA_X_GDME_VT" {
            return Ok(XcFuncId::MGGA_X_GDME_VT);
        }
        if lookup == "LDA_X_SLOC" {
            return Ok(XcFuncId::LDA_X_SLOC);
        }
        if lookup == "MGGA_X_REVTM" {
            return Ok(XcFuncId::MGGA_X_REVTM);
        }
        if lookup == "MGGA_C_REVTM" {
            return Ok(XcFuncId::MGGA_C_REVTM);
        }
        if lookup == "HYB_MGGA_XC_EDMGGAH" {
            return Ok(XcFuncId::HYB_MGGA_XC_EDMGGAH);
        }
        if lookup == "MGGA_X_MBRXC_BG" {
            return Ok(XcFuncId::MGGA_X_MBRXC_BG);
        }
        if lookup == "MGGA_X_MBRXH_BG" {
            return Ok(XcFuncId::MGGA_X_MBRXH_BG);
        }
        if lookup == "MGGA_X_HLTA" {
            return Ok(XcFuncId::MGGA_X_HLTA);
        }
        if lookup == "MGGA_C_HLTAPW" {
            return Ok(XcFuncId::MGGA_C_HLTAPW);
        }
        if lookup == "MGGA_X_SCANL" {
            return Ok(XcFuncId::MGGA_X_SCANL);
        }
        if lookup == "MGGA_X_REVSCANL" {
            return Ok(XcFuncId::MGGA_X_REVSCANL);
        }
        if lookup == "MGGA_C_SCANL" {
            return Ok(XcFuncId::MGGA_C_SCANL);
        }
        if lookup == "MGGA_C_SCANL_RVV10" {
            return Ok(XcFuncId::MGGA_C_SCANL_RVV10);
        }
        if lookup == "MGGA_C_SCANL_VV10" {
            return Ok(XcFuncId::MGGA_C_SCANL_VV10);
        }
        if lookup == "HYB_MGGA_X_JS18" {
            return Ok(XcFuncId::HYB_MGGA_X_JS18);
        }
        if lookup == "HYB_MGGA_X_PJS18" {
            return Ok(XcFuncId::HYB_MGGA_X_PJS18);
        }
        if lookup == "MGGA_X_TASK" {
            return Ok(XcFuncId::MGGA_X_TASK);
        }
        if lookup == "MGGA_X_MGGAC" {
            return Ok(XcFuncId::MGGA_X_MGGAC);
        }
        if lookup == "GGA_C_MGGAC" {
            return Ok(XcFuncId::GGA_C_MGGAC);
        }
        if lookup == "MGGA_X_MBR" {
            return Ok(XcFuncId::MGGA_X_MBR);
        }
        if lookup == "MGGA_X_R2SCANL" {
            return Ok(XcFuncId::MGGA_X_R2SCANL);
        }
        if lookup == "MGGA_C_R2SCANL" {
            return Ok(XcFuncId::MGGA_C_R2SCANL);
        }
        if lookup == "HYB_MGGA_XC_LC_TMLYP" {
            return Ok(XcFuncId::HYB_MGGA_XC_LC_TMLYP);
        }
        if lookup == "MGGA_X_MTASK" {
            return Ok(XcFuncId::MGGA_X_MTASK);
        }
        if lookup == "GGA_X_Q1D" {
            return Ok(XcFuncId::GGA_X_Q1D);
        }
        if lookup == "MGGA_X_KTBM_0" {
            return Ok(XcFuncId::MGGA_X_KTBM_0);
        }
        if lookup == "MGGA_X_KTBM_1" {
            return Ok(XcFuncId::MGGA_X_KTBM_1);
        }
        if lookup == "MGGA_X_KTBM_2" {
            return Ok(XcFuncId::MGGA_X_KTBM_2);
        }
        if lookup == "MGGA_X_KTBM_3" {
            return Ok(XcFuncId::MGGA_X_KTBM_3);
        }
        if lookup == "MGGA_X_KTBM_4" {
            return Ok(XcFuncId::MGGA_X_KTBM_4);
        }
        if lookup == "MGGA_X_KTBM_5" {
            return Ok(XcFuncId::MGGA_X_KTBM_5);
        }
        if lookup == "MGGA_X_KTBM_6" {
            return Ok(XcFuncId::MGGA_X_KTBM_6);
        }
        if lookup == "MGGA_X_KTBM_7" {
            return Ok(XcFuncId::MGGA_X_KTBM_7);
        }
        if lookup == "MGGA_X_KTBM_8" {
            return Ok(XcFuncId::MGGA_X_KTBM_8);
        }
        if lookup == "MGGA_X_KTBM_9" {
            return Ok(XcFuncId::MGGA_X_KTBM_9);
        }
        if lookup == "MGGA_X_KTBM_10" {
            return Ok(XcFuncId::MGGA_X_KTBM_10);
        }
        if lookup == "MGGA_X_KTBM_11" {
            return Ok(XcFuncId::MGGA_X_KTBM_11);
        }
        if lookup == "MGGA_X_KTBM_12" {
            return Ok(XcFuncId::MGGA_X_KTBM_12);
        }
        if lookup == "MGGA_X_KTBM_13" {
            return Ok(XcFuncId::MGGA_X_KTBM_13);
        }
        if lookup == "MGGA_X_KTBM_14" {
            return Ok(XcFuncId::MGGA_X_KTBM_14);
        }
        if lookup == "MGGA_X_KTBM_15" {
            return Ok(XcFuncId::MGGA_X_KTBM_15);
        }
        if lookup == "MGGA_X_KTBM_16" {
            return Ok(XcFuncId::MGGA_X_KTBM_16);
        }
        if lookup == "MGGA_X_KTBM_17" {
            return Ok(XcFuncId::MGGA_X_KTBM_17);
        }
        if lookup == "MGGA_X_KTBM_18" {
            return Ok(XcFuncId::MGGA_X_KTBM_18);
        }
        if lookup == "MGGA_X_KTBM_19" {
            return Ok(XcFuncId::MGGA_X_KTBM_19);
        }
        if lookup == "MGGA_X_KTBM_20" {
            return Ok(XcFuncId::MGGA_X_KTBM_20);
        }
        if lookup == "MGGA_X_KTBM_21" {
            return Ok(XcFuncId::MGGA_X_KTBM_21);
        }
        if lookup == "MGGA_X_KTBM_22" {
            return Ok(XcFuncId::MGGA_X_KTBM_22);
        }
        if lookup == "MGGA_X_KTBM_23" {
            return Ok(XcFuncId::MGGA_X_KTBM_23);
        }
        if lookup == "MGGA_X_KTBM_24" {
            return Ok(XcFuncId::MGGA_X_KTBM_24);
        }
        if lookup == "MGGA_X_KTBM_GAP" {
            return Ok(XcFuncId::MGGA_X_KTBM_GAP);
        }
        if lookup == "MGGA_X_MSPBEL" {
            return Ok(XcFuncId::MGGA_X_MSPBEL);
        }
        if lookup == "MGGA_X_RMSPBEL" {
            return Ok(XcFuncId::MGGA_X_RMSPBEL);
        }
        if lookup == "MGGA_X_MSRPBEL" {
            return Ok(XcFuncId::MGGA_X_MSRPBEL);
        }
        if lookup == "MGGA_X_RMSRPBEL" {
            return Ok(XcFuncId::MGGA_X_RMSRPBEL);
        }
        if lookup == "MGGA_X_MSB86BL" {
            return Ok(XcFuncId::MGGA_X_MSB86BL);
        }
        if lookup == "MGGA_X_RMSB86BL" {
            return Ok(XcFuncId::MGGA_X_RMSB86BL);
        }
        if lookup == "HYB_MGGA_X_PI_M06_2X_DL" {
            return Ok(XcFuncId::HYB_MGGA_X_PI_M06_2X_DL);
        }
        if lookup == "MGGA_C_PI_M06_2X_DL" {
            return Ok(XcFuncId::MGGA_C_PI_M06_2X_DL);
        }
        if lookup == "HYB_MGGA_X_PI_M06_2X" {
            return Ok(XcFuncId::HYB_MGGA_X_PI_M06_2X);
        }
        if lookup == "MGGA_C_PI_M06_2X" {
            return Ok(XcFuncId::MGGA_C_PI_M06_2X);
        }
        Err(format!("Unknown libxc functional: {s}"))
    }
}

impl From<XcFuncId> for u32 {
    fn from(id: XcFuncId) -> u32 {
        id as u32
    }
}

impl TryFrom<u32> for XcFuncId {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(XcFuncId::LDA_X),
            2 => Ok(XcFuncId::LDA_C_WIGNER),
            3 => Ok(XcFuncId::LDA_C_RPA),
            4 => Ok(XcFuncId::LDA_C_HL),
            5 => Ok(XcFuncId::LDA_C_GL),
            6 => Ok(XcFuncId::LDA_C_XALPHA),
            7 => Ok(XcFuncId::LDA_C_VWN),
            8 => Ok(XcFuncId::LDA_C_VWN_RPA),
            9 => Ok(XcFuncId::LDA_C_PZ),
            10 => Ok(XcFuncId::LDA_C_PZ_MOD),
            11 => Ok(XcFuncId::LDA_C_OB_PZ),
            12 => Ok(XcFuncId::LDA_C_PW),
            13 => Ok(XcFuncId::LDA_C_PW_MOD),
            14 => Ok(XcFuncId::LDA_C_OB_PW),
            15 => Ok(XcFuncId::LDA_C_2D_AMGB),
            16 => Ok(XcFuncId::LDA_C_2D_PRM),
            17 => Ok(XcFuncId::LDA_C_VBH),
            18 => Ok(XcFuncId::LDA_C_1D_CSS),
            19 => Ok(XcFuncId::LDA_X_2D),
            20 => Ok(XcFuncId::LDA_XC_TETER93),
            21 => Ok(XcFuncId::LDA_X_1D_SOFT),
            22 => Ok(XcFuncId::LDA_C_ML1),
            23 => Ok(XcFuncId::LDA_C_ML2),
            24 => Ok(XcFuncId::LDA_C_GOMBAS),
            25 => Ok(XcFuncId::LDA_C_PW_RPA),
            26 => Ok(XcFuncId::LDA_C_1D_LOOS),
            27 => Ok(XcFuncId::LDA_C_RC04),
            28 => Ok(XcFuncId::LDA_C_VWN_1),
            29 => Ok(XcFuncId::LDA_C_VWN_2),
            30 => Ok(XcFuncId::LDA_C_VWN_3),
            31 => Ok(XcFuncId::LDA_C_VWN_4),
            32 => Ok(XcFuncId::GGA_X_GAM),
            33 => Ok(XcFuncId::GGA_C_GAM),
            34 => Ok(XcFuncId::GGA_X_HCTH_A),
            35 => Ok(XcFuncId::GGA_X_EV93),
            36 => Ok(XcFuncId::HYB_MGGA_X_DLDF),
            37 => Ok(XcFuncId::MGGA_C_DLDF),
            38 => Ok(XcFuncId::GGA_X_BCGP),
            39 => Ok(XcFuncId::GGA_C_ACGGA),
            40 => Ok(XcFuncId::GGA_X_LAMBDA_OC2_N),
            41 => Ok(XcFuncId::GGA_X_B86_R),
            42 => Ok(XcFuncId::MGGA_XC_ZLP),
            43 => Ok(XcFuncId::LDA_XC_ZLP),
            44 => Ok(XcFuncId::GGA_X_LAMBDA_CH_N),
            45 => Ok(XcFuncId::GGA_X_LAMBDA_LO_N),
            46 => Ok(XcFuncId::GGA_X_HJS_B88_V2),
            47 => Ok(XcFuncId::GGA_C_Q2D),
            48 => Ok(XcFuncId::GGA_X_Q2D),
            49 => Ok(XcFuncId::GGA_X_PBE_MOL),
            50 => Ok(XcFuncId::LDA_K_TF),
            51 => Ok(XcFuncId::LDA_K_LP),
            52 => Ok(XcFuncId::GGA_K_TFVW),
            53 => Ok(XcFuncId::GGA_K_REVAPBEINT),
            54 => Ok(XcFuncId::GGA_K_APBEINT),
            55 => Ok(XcFuncId::GGA_K_REVAPBE),
            56 => Ok(XcFuncId::GGA_X_AK13),
            57 => Ok(XcFuncId::GGA_K_MEYER),
            58 => Ok(XcFuncId::GGA_X_LV_RPW86),
            59 => Ok(XcFuncId::GGA_X_PBE_TCA),
            60 => Ok(XcFuncId::GGA_X_PBEINT),
            61 => Ok(XcFuncId::GGA_C_ZPBEINT),
            62 => Ok(XcFuncId::GGA_C_PBEINT),
            63 => Ok(XcFuncId::GGA_C_ZPBESOL),
            64 => Ok(XcFuncId::MGGA_XC_OTPSS_D),
            65 => Ok(XcFuncId::GGA_XC_OPBE_D),
            66 => Ok(XcFuncId::GGA_XC_OPWLYP_D),
            67 => Ok(XcFuncId::GGA_XC_OBLYP_D),
            68 => Ok(XcFuncId::GGA_X_VMT84_GE),
            69 => Ok(XcFuncId::GGA_X_VMT84_PBE),
            70 => Ok(XcFuncId::GGA_X_VMT_GE),
            71 => Ok(XcFuncId::GGA_X_VMT_PBE),
            72 => Ok(XcFuncId::MGGA_C_CS),
            73 => Ok(XcFuncId::MGGA_C_MN12_SX),
            74 => Ok(XcFuncId::MGGA_C_MN12_L),
            75 => Ok(XcFuncId::MGGA_C_M11_L),
            76 => Ok(XcFuncId::MGGA_C_M11),
            77 => Ok(XcFuncId::MGGA_C_M08_SO),
            78 => Ok(XcFuncId::MGGA_C_M08_HX),
            79 => Ok(XcFuncId::GGA_C_N12_SX),
            80 => Ok(XcFuncId::GGA_C_N12),
            81 => Ok(XcFuncId::HYB_GGA_X_N12_SX),
            82 => Ok(XcFuncId::GGA_X_N12),
            83 => Ok(XcFuncId::GGA_C_REGTPSS),
            84 => Ok(XcFuncId::GGA_C_OP_XALPHA),
            85 => Ok(XcFuncId::GGA_C_OP_G96),
            86 => Ok(XcFuncId::GGA_C_OP_PBE),
            87 => Ok(XcFuncId::GGA_C_OP_B88),
            88 => Ok(XcFuncId::GGA_C_FT97),
            89 => Ok(XcFuncId::GGA_C_SPBE),
            90 => Ok(XcFuncId::GGA_X_SSB_SW),
            91 => Ok(XcFuncId::GGA_X_SSB),
            92 => Ok(XcFuncId::GGA_X_SSB_D),
            93 => Ok(XcFuncId::GGA_XC_HCTH_407P),
            94 => Ok(XcFuncId::GGA_XC_HCTH_P76),
            95 => Ok(XcFuncId::GGA_XC_HCTH_P14),
            96 => Ok(XcFuncId::GGA_XC_B97_GGA1),
            97 => Ok(XcFuncId::GGA_C_HCTH_A),
            98 => Ok(XcFuncId::GGA_X_BPCCAC),
            99 => Ok(XcFuncId::GGA_C_REVTCA),
            100 => Ok(XcFuncId::GGA_C_TCA),
            101 => Ok(XcFuncId::GGA_X_PBE),
            102 => Ok(XcFuncId::GGA_X_PBE_R),
            103 => Ok(XcFuncId::GGA_X_B86),
            104 => Ok(XcFuncId::HYB_LDA_XC_B93),
            105 => Ok(XcFuncId::GGA_X_B86_MGC),
            106 => Ok(XcFuncId::GGA_X_B88),
            107 => Ok(XcFuncId::GGA_X_G96),
            108 => Ok(XcFuncId::GGA_X_PW86),
            109 => Ok(XcFuncId::GGA_X_PW91),
            110 => Ok(XcFuncId::GGA_X_OPTX),
            111 => Ok(XcFuncId::GGA_X_DK87_R1),
            112 => Ok(XcFuncId::GGA_X_DK87_R2),
            113 => Ok(XcFuncId::GGA_X_LG93),
            114 => Ok(XcFuncId::GGA_X_FT97_A),
            115 => Ok(XcFuncId::GGA_X_FT97_B),
            116 => Ok(XcFuncId::GGA_X_PBE_SOL),
            117 => Ok(XcFuncId::GGA_X_RPBE),
            118 => Ok(XcFuncId::GGA_X_WC),
            119 => Ok(XcFuncId::GGA_X_MPW91),
            120 => Ok(XcFuncId::GGA_X_AM05),
            121 => Ok(XcFuncId::GGA_X_PBEA),
            122 => Ok(XcFuncId::GGA_X_MPBE),
            123 => Ok(XcFuncId::GGA_X_XPBE),
            124 => Ok(XcFuncId::GGA_X_2D_B86_MGC),
            125 => Ok(XcFuncId::GGA_X_BAYESIAN),
            126 => Ok(XcFuncId::GGA_X_PBE_JSJR),
            127 => Ok(XcFuncId::GGA_X_2D_B88),
            128 => Ok(XcFuncId::GGA_X_2D_B86),
            129 => Ok(XcFuncId::GGA_X_2D_PBE),
            130 => Ok(XcFuncId::GGA_C_PBE),
            131 => Ok(XcFuncId::GGA_C_LYP),
            132 => Ok(XcFuncId::GGA_C_P86),
            133 => Ok(XcFuncId::GGA_C_PBE_SOL),
            134 => Ok(XcFuncId::GGA_C_PW91),
            135 => Ok(XcFuncId::GGA_C_AM05),
            136 => Ok(XcFuncId::GGA_C_XPBE),
            137 => Ok(XcFuncId::GGA_C_LM),
            138 => Ok(XcFuncId::GGA_C_PBE_JRGX),
            139 => Ok(XcFuncId::GGA_X_OPTB88_VDW),
            140 => Ok(XcFuncId::GGA_X_PBEK1_VDW),
            141 => Ok(XcFuncId::GGA_X_OPTPBE_VDW),
            142 => Ok(XcFuncId::GGA_X_RGE2),
            143 => Ok(XcFuncId::GGA_C_RGE2),
            144 => Ok(XcFuncId::GGA_X_RPW86),
            145 => Ok(XcFuncId::GGA_X_KT1),
            146 => Ok(XcFuncId::GGA_XC_KT2),
            147 => Ok(XcFuncId::GGA_C_WL),
            148 => Ok(XcFuncId::GGA_C_WI),
            149 => Ok(XcFuncId::GGA_X_MB88),
            150 => Ok(XcFuncId::GGA_X_SOGGA),
            151 => Ok(XcFuncId::GGA_X_SOGGA11),
            152 => Ok(XcFuncId::GGA_C_SOGGA11),
            153 => Ok(XcFuncId::GGA_C_WI0),
            154 => Ok(XcFuncId::GGA_XC_TH1),
            155 => Ok(XcFuncId::GGA_XC_TH2),
            156 => Ok(XcFuncId::GGA_XC_TH3),
            157 => Ok(XcFuncId::GGA_XC_TH4),
            158 => Ok(XcFuncId::GGA_X_C09X),
            159 => Ok(XcFuncId::GGA_C_SOGGA11_X),
            160 => Ok(XcFuncId::GGA_X_LB),
            161 => Ok(XcFuncId::GGA_XC_HCTH_93),
            162 => Ok(XcFuncId::GGA_XC_HCTH_120),
            163 => Ok(XcFuncId::GGA_XC_HCTH_147),
            164 => Ok(XcFuncId::GGA_XC_HCTH_407),
            165 => Ok(XcFuncId::GGA_XC_EDF1),
            166 => Ok(XcFuncId::GGA_XC_XLYP),
            167 => Ok(XcFuncId::GGA_XC_KT1),
            168 => Ok(XcFuncId::GGA_X_LSPBE),
            169 => Ok(XcFuncId::GGA_X_LSRPBE),
            170 => Ok(XcFuncId::GGA_XC_B97_D),
            171 => Ok(XcFuncId::GGA_X_OPTB86B_VDW),
            172 => Ok(XcFuncId::MGGA_C_REVM11),
            173 => Ok(XcFuncId::GGA_XC_PBE1W),
            174 => Ok(XcFuncId::GGA_XC_MPWLYP1W),
            175 => Ok(XcFuncId::GGA_XC_PBELYP1W),
            176 => Ok(XcFuncId::GGA_C_ACGGAP),
            177 => Ok(XcFuncId::HYB_LDA_XC_LDA0),
            178 => Ok(XcFuncId::HYB_LDA_XC_CAM_LDA0),
            179 => Ok(XcFuncId::GGA_X_B88_6311G),
            180 => Ok(XcFuncId::GGA_X_NCAP),
            181 => Ok(XcFuncId::GGA_XC_NCAP),
            182 => Ok(XcFuncId::GGA_X_LBM),
            183 => Ok(XcFuncId::GGA_X_OL2),
            184 => Ok(XcFuncId::GGA_X_APBE),
            185 => Ok(XcFuncId::GGA_K_APBE),
            186 => Ok(XcFuncId::GGA_C_APBE),
            187 => Ok(XcFuncId::GGA_K_TW1),
            188 => Ok(XcFuncId::GGA_K_TW2),
            189 => Ok(XcFuncId::GGA_K_TW3),
            190 => Ok(XcFuncId::GGA_K_TW4),
            191 => Ok(XcFuncId::GGA_X_HTBS),
            192 => Ok(XcFuncId::GGA_X_AIRY),
            193 => Ok(XcFuncId::GGA_X_LAG),
            194 => Ok(XcFuncId::GGA_XC_MOHLYP),
            195 => Ok(XcFuncId::GGA_XC_MOHLYP2),
            196 => Ok(XcFuncId::LDA_XC_TH_FL),
            197 => Ok(XcFuncId::GGA_XC_TH_FC),
            198 => Ok(XcFuncId::GGA_XC_TH_FCFO),
            199 => Ok(XcFuncId::GGA_XC_TH_FCO),
            200 => Ok(XcFuncId::GGA_C_OPTC),
            201 => Ok(XcFuncId::MGGA_X_LTA),
            202 => Ok(XcFuncId::MGGA_X_TPSS),
            203 => Ok(XcFuncId::MGGA_X_M06_L),
            204 => Ok(XcFuncId::MGGA_X_GVT4),
            205 => Ok(XcFuncId::MGGA_X_TAU_HCTH),
            206 => Ok(XcFuncId::MGGA_X_BR89),
            207 => Ok(XcFuncId::MGGA_X_BJ06),
            208 => Ok(XcFuncId::MGGA_X_TB09),
            209 => Ok(XcFuncId::MGGA_X_RPP09),
            210 => Ok(XcFuncId::MGGA_X_2D_PRHG07),
            211 => Ok(XcFuncId::MGGA_X_2D_PRHG07_PRP10),
            212 => Ok(XcFuncId::MGGA_X_REVTPSS),
            213 => Ok(XcFuncId::MGGA_X_PKZB),
            214 => Ok(XcFuncId::MGGA_X_BR89_1),
            215 => Ok(XcFuncId::GGA_X_ECMV92),
            216 => Ok(XcFuncId::GGA_C_PBE_VWN),
            217 => Ok(XcFuncId::GGA_C_P86_FT),
            218 => Ok(XcFuncId::GGA_K_RATIONAL_P),
            219 => Ok(XcFuncId::GGA_K_PG1),
            220 => Ok(XcFuncId::MGGA_K_PGSL025),
            221 => Ok(XcFuncId::MGGA_X_MS0),
            222 => Ok(XcFuncId::MGGA_X_MS1),
            223 => Ok(XcFuncId::MGGA_X_MS2),
            224 => Ok(XcFuncId::HYB_MGGA_X_MS2H),
            225 => Ok(XcFuncId::MGGA_X_TH),
            226 => Ok(XcFuncId::MGGA_X_M11_L),
            227 => Ok(XcFuncId::MGGA_X_MN12_L),
            228 => Ok(XcFuncId::MGGA_X_MS2_REV),
            229 => Ok(XcFuncId::MGGA_XC_CC06),
            230 => Ok(XcFuncId::MGGA_X_GP86),
            231 => Ok(XcFuncId::MGGA_C_TPSS),
            232 => Ok(XcFuncId::MGGA_C_VSXC),
            233 => Ok(XcFuncId::MGGA_C_M06_L),
            234 => Ok(XcFuncId::MGGA_C_M06_HF),
            235 => Ok(XcFuncId::MGGA_C_M06),
            236 => Ok(XcFuncId::MGGA_C_M06_2X),
            237 => Ok(XcFuncId::MGGA_C_M05),
            238 => Ok(XcFuncId::MGGA_C_M05_2X),
            239 => Ok(XcFuncId::MGGA_C_PKZB),
            240 => Ok(XcFuncId::MGGA_C_BC95),
            241 => Ok(XcFuncId::MGGA_C_REVTPSS),
            242 => Ok(XcFuncId::MGGA_XC_TPSSLYP1W),
            243 => Ok(XcFuncId::MGGA_X_MK00B),
            244 => Ok(XcFuncId::MGGA_X_BLOC),
            245 => Ok(XcFuncId::MGGA_X_MODTPSS),
            246 => Ok(XcFuncId::GGA_C_PBELOC),
            247 => Ok(XcFuncId::MGGA_C_TPSSLOC),
            248 => Ok(XcFuncId::HYB_MGGA_X_MN12_SX),
            249 => Ok(XcFuncId::MGGA_X_MBEEF),
            250 => Ok(XcFuncId::MGGA_X_MBEEFVDW),
            251 => Ok(XcFuncId::MGGA_C_TM),
            252 => Ok(XcFuncId::GGA_C_P86VWN),
            253 => Ok(XcFuncId::GGA_C_P86VWN_FT),
            254 => Ok(XcFuncId::MGGA_XC_B97M_V),
            255 => Ok(XcFuncId::GGA_XC_VV10),
            256 => Ok(XcFuncId::MGGA_X_JK),
            257 => Ok(XcFuncId::MGGA_X_MVS),
            258 => Ok(XcFuncId::GGA_C_PBEFE),
            259 => Ok(XcFuncId::LDA_XC_KSDT),
            260 => Ok(XcFuncId::MGGA_X_MN15_L),
            261 => Ok(XcFuncId::MGGA_C_MN15_L),
            262 => Ok(XcFuncId::GGA_C_OP_PW91),
            263 => Ok(XcFuncId::MGGA_X_SCAN),
            264 => Ok(XcFuncId::HYB_MGGA_X_SCAN0),
            265 => Ok(XcFuncId::GGA_X_PBEFE),
            266 => Ok(XcFuncId::HYB_GGA_XC_B97_1P),
            267 => Ok(XcFuncId::MGGA_C_SCAN),
            268 => Ok(XcFuncId::HYB_MGGA_X_MN15),
            269 => Ok(XcFuncId::MGGA_C_MN15),
            270 => Ok(XcFuncId::GGA_X_CAP),
            271 => Ok(XcFuncId::GGA_X_EB88),
            272 => Ok(XcFuncId::GGA_C_PBE_MOL),
            273 => Ok(XcFuncId::HYB_GGA_XC_PBE_MOL0),
            274 => Ok(XcFuncId::HYB_GGA_XC_PBE_SOL0),
            275 => Ok(XcFuncId::HYB_GGA_XC_PBEB0),
            276 => Ok(XcFuncId::HYB_GGA_XC_PBE_MOLB0),
            277 => Ok(XcFuncId::GGA_K_ABSP3),
            278 => Ok(XcFuncId::GGA_K_ABSP4),
            279 => Ok(XcFuncId::HYB_MGGA_X_BMK),
            280 => Ok(XcFuncId::GGA_C_BMK),
            281 => Ok(XcFuncId::GGA_C_TAU_HCTH),
            282 => Ok(XcFuncId::HYB_MGGA_X_TAU_HCTH),
            283 => Ok(XcFuncId::GGA_C_HYB_TAU_HCTH),
            284 => Ok(XcFuncId::MGGA_X_B00),
            285 => Ok(XcFuncId::GGA_X_BEEFVDW),
            286 => Ok(XcFuncId::GGA_XC_BEEFVDW),
            287 => Ok(XcFuncId::LDA_C_CHACHIYO),
            288 => Ok(XcFuncId::MGGA_XC_HLE17),
            289 => Ok(XcFuncId::LDA_C_LP96),
            290 => Ok(XcFuncId::HYB_GGA_XC_PBE50),
            291 => Ok(XcFuncId::GGA_X_PBETRANS),
            292 => Ok(XcFuncId::MGGA_C_SCAN_RVV10),
            293 => Ok(XcFuncId::MGGA_X_REVM06_L),
            294 => Ok(XcFuncId::MGGA_C_REVM06_L),
            295 => Ok(XcFuncId::HYB_MGGA_X_M08_HX),
            296 => Ok(XcFuncId::HYB_MGGA_X_M08_SO),
            297 => Ok(XcFuncId::HYB_MGGA_X_M11),
            298 => Ok(XcFuncId::GGA_X_CHACHIYO),
            299 => Ok(XcFuncId::MGGA_X_RTPSS),
            300 => Ok(XcFuncId::MGGA_X_MS2B),
            301 => Ok(XcFuncId::MGGA_X_MS2BS),
            302 => Ok(XcFuncId::MGGA_X_MVSB),
            303 => Ok(XcFuncId::MGGA_X_MVSBS),
            304 => Ok(XcFuncId::HYB_MGGA_X_REVM11),
            305 => Ok(XcFuncId::HYB_MGGA_X_REVM06),
            306 => Ok(XcFuncId::MGGA_C_REVM06),
            307 => Ok(XcFuncId::LDA_C_CHACHIYO_MOD),
            308 => Ok(XcFuncId::LDA_C_KARASIEV_MOD),
            309 => Ok(XcFuncId::GGA_C_CHACHIYO),
            310 => Ok(XcFuncId::HYB_MGGA_X_M06_SX),
            311 => Ok(XcFuncId::MGGA_C_M06_SX),
            312 => Ok(XcFuncId::GGA_X_REVSSB_D),
            313 => Ok(XcFuncId::GGA_C_CCDF),
            314 => Ok(XcFuncId::HYB_GGA_XC_HFLYP),
            315 => Ok(XcFuncId::HYB_GGA_XC_B3P86_NWCHEM),
            316 => Ok(XcFuncId::GGA_X_PW91_MOD),
            317 => Ok(XcFuncId::LDA_C_W20),
            318 => Ok(XcFuncId::LDA_XC_CORRKSDT),
            319 => Ok(XcFuncId::MGGA_X_FT98),
            320 => Ok(XcFuncId::GGA_X_PBE_MOD),
            321 => Ok(XcFuncId::GGA_X_PBE_GAUSSIAN),
            322 => Ok(XcFuncId::GGA_C_PBE_GAUSSIAN),
            323 => Ok(XcFuncId::MGGA_C_TPSS_GAUSSIAN),
            324 => Ok(XcFuncId::GGA_X_NCAPR),
            325 => Ok(XcFuncId::HYB_GGA_XC_RELPBE0),
            326 => Ok(XcFuncId::MGGA_X_EEL),
            327 => Ok(XcFuncId::GGA_XC_B97_3C),
            328 => Ok(XcFuncId::LDA_C_EPC17),
            329 => Ok(XcFuncId::LDA_C_EPC17_2),
            330 => Ok(XcFuncId::LDA_C_EPC18_1),
            331 => Ok(XcFuncId::LDA_C_EPC18_2),
            332 => Ok(XcFuncId::GGA_XC_DLB97),
            333 => Ok(XcFuncId::MGGA_X_MSCAN),
            334 => Ok(XcFuncId::MGGA_C_MSCAN),
            335 => Ok(XcFuncId::GGA_X_T_PBE1),
            336 => Ok(XcFuncId::GGA_X_T_PBE2),
            337 => Ok(XcFuncId::LDA_X_T_SLOC),
            338 => Ok(XcFuncId::GGA_X_BKL1),
            339 => Ok(XcFuncId::GGA_X_BKL2),
            340 => Ok(XcFuncId::HYB_MGGA_X_CF22D),
            341 => Ok(XcFuncId::MGGA_C_CF22D),
            342 => Ok(XcFuncId::MGGA_X_LAK),
            343 => Ok(XcFuncId::GGA_C_BKL1),
            344 => Ok(XcFuncId::GGA_C_BKL2),
            345 => Ok(XcFuncId::MGGA_C_LAK),
            346 => Ok(XcFuncId::GGA_X_DF3_OPT1),
            347 => Ok(XcFuncId::GGA_X_DF3_OPT2),
            385 => Ok(XcFuncId::HYB_GGA_XC_CQTP25),
            386 => Ok(XcFuncId::HYB_GGA_XC_OPB3LYP),
            387 => Ok(XcFuncId::MGGA_C_CC),
            388 => Ok(XcFuncId::MGGA_C_CCALDA),
            389 => Ok(XcFuncId::HYB_MGGA_XC_BR3P86),
            390 => Ok(XcFuncId::HYB_GGA_XC_CASE21),
            391 => Ok(XcFuncId::MGGA_C_RREGTM),
            392 => Ok(XcFuncId::HYB_GGA_XC_PBE_2X),
            393 => Ok(XcFuncId::HYB_GGA_XC_PBE38),
            394 => Ok(XcFuncId::HYB_GGA_XC_B3LYP3),
            395 => Ok(XcFuncId::HYB_GGA_XC_CAM_O3LYP),
            396 => Ok(XcFuncId::HYB_MGGA_XC_TPSS0),
            397 => Ok(XcFuncId::MGGA_C_B94),
            398 => Ok(XcFuncId::HYB_MGGA_XC_B94_HYB),
            399 => Ok(XcFuncId::HYB_GGA_XC_WB97X_D3),
            400 => Ok(XcFuncId::HYB_GGA_XC_LC_BLYP),
            401 => Ok(XcFuncId::HYB_GGA_XC_B3PW91),
            402 => Ok(XcFuncId::HYB_GGA_XC_B3LYP),
            403 => Ok(XcFuncId::HYB_GGA_XC_B3P86),
            404 => Ok(XcFuncId::HYB_GGA_XC_O3LYP),
            405 => Ok(XcFuncId::HYB_GGA_XC_MPW1K),
            406 => Ok(XcFuncId::HYB_GGA_XC_PBEH),
            407 => Ok(XcFuncId::HYB_GGA_XC_B97),
            408 => Ok(XcFuncId::HYB_GGA_XC_B97_1),
            409 => Ok(XcFuncId::HYB_GGA_XC_APF),
            410 => Ok(XcFuncId::HYB_GGA_XC_B97_2),
            411 => Ok(XcFuncId::HYB_GGA_XC_X3LYP),
            412 => Ok(XcFuncId::HYB_GGA_XC_B1WC),
            413 => Ok(XcFuncId::HYB_GGA_XC_B97_K),
            414 => Ok(XcFuncId::HYB_GGA_XC_B97_3),
            415 => Ok(XcFuncId::HYB_GGA_XC_MPW3PW),
            416 => Ok(XcFuncId::HYB_GGA_XC_B1LYP),
            417 => Ok(XcFuncId::HYB_GGA_XC_B1PW91),
            418 => Ok(XcFuncId::HYB_GGA_XC_MPW1PW),
            419 => Ok(XcFuncId::HYB_GGA_XC_MPW3LYP),
            420 => Ok(XcFuncId::HYB_GGA_XC_SB98_1A),
            421 => Ok(XcFuncId::HYB_GGA_XC_SB98_1B),
            422 => Ok(XcFuncId::HYB_GGA_XC_SB98_1C),
            423 => Ok(XcFuncId::HYB_GGA_XC_SB98_2A),
            424 => Ok(XcFuncId::HYB_GGA_XC_SB98_2B),
            425 => Ok(XcFuncId::HYB_GGA_XC_SB98_2C),
            426 => Ok(XcFuncId::HYB_GGA_X_SOGGA11_X),
            427 => Ok(XcFuncId::HYB_GGA_XC_HSE03),
            428 => Ok(XcFuncId::HYB_GGA_XC_HSE06),
            429 => Ok(XcFuncId::HYB_GGA_XC_HJS_PBE),
            430 => Ok(XcFuncId::HYB_GGA_XC_HJS_PBE_SOL),
            431 => Ok(XcFuncId::HYB_GGA_XC_HJS_B88),
            432 => Ok(XcFuncId::HYB_GGA_XC_HJS_B97X),
            433 => Ok(XcFuncId::HYB_GGA_XC_CAM_B3LYP),
            434 => Ok(XcFuncId::HYB_GGA_XC_TUNED_CAM_B3LYP),
            435 => Ok(XcFuncId::HYB_GGA_XC_BHANDH),
            436 => Ok(XcFuncId::HYB_GGA_XC_BHANDHLYP),
            437 => Ok(XcFuncId::HYB_GGA_XC_MB3LYP_RC04),
            438 => Ok(XcFuncId::HYB_MGGA_X_M05),
            439 => Ok(XcFuncId::HYB_MGGA_X_M05_2X),
            440 => Ok(XcFuncId::HYB_MGGA_XC_B88B95),
            441 => Ok(XcFuncId::HYB_MGGA_XC_B86B95),
            442 => Ok(XcFuncId::HYB_MGGA_XC_PW86B95),
            443 => Ok(XcFuncId::HYB_MGGA_XC_BB1K),
            444 => Ok(XcFuncId::HYB_MGGA_X_M06_HF),
            445 => Ok(XcFuncId::HYB_MGGA_XC_MPW1B95),
            446 => Ok(XcFuncId::HYB_MGGA_XC_MPWB1K),
            447 => Ok(XcFuncId::HYB_MGGA_XC_X1B95),
            448 => Ok(XcFuncId::HYB_MGGA_XC_XB1K),
            449 => Ok(XcFuncId::HYB_MGGA_X_M06),
            450 => Ok(XcFuncId::HYB_MGGA_X_M06_2X),
            451 => Ok(XcFuncId::HYB_MGGA_XC_PW6B95),
            452 => Ok(XcFuncId::HYB_MGGA_XC_PWB6K),
            453 => Ok(XcFuncId::HYB_GGA_XC_MPWLYP1M),
            454 => Ok(XcFuncId::HYB_GGA_XC_REVB3LYP),
            455 => Ok(XcFuncId::HYB_GGA_XC_CAMY_BLYP),
            456 => Ok(XcFuncId::HYB_GGA_XC_PBE0_13),
            457 => Ok(XcFuncId::HYB_MGGA_XC_TPSSH),
            458 => Ok(XcFuncId::HYB_MGGA_XC_REVTPSSH),
            459 => Ok(XcFuncId::HYB_GGA_XC_B3LYPS),
            460 => Ok(XcFuncId::HYB_GGA_XC_QTP17),
            461 => Ok(XcFuncId::HYB_GGA_XC_B3LYP_MCM1),
            462 => Ok(XcFuncId::HYB_GGA_XC_B3LYP_MCM2),
            463 => Ok(XcFuncId::HYB_GGA_XC_WB97),
            464 => Ok(XcFuncId::HYB_GGA_XC_WB97X),
            465 => Ok(XcFuncId::HYB_GGA_XC_LRC_WPBEH),
            466 => Ok(XcFuncId::HYB_GGA_XC_WB97X_V),
            467 => Ok(XcFuncId::HYB_GGA_XC_LCY_PBE),
            468 => Ok(XcFuncId::HYB_GGA_XC_LCY_BLYP),
            469 => Ok(XcFuncId::HYB_GGA_XC_LC_VV10),
            470 => Ok(XcFuncId::HYB_GGA_XC_CAMY_B3LYP),
            471 => Ok(XcFuncId::HYB_GGA_XC_WB97X_D),
            472 => Ok(XcFuncId::HYB_GGA_XC_HPBEINT),
            473 => Ok(XcFuncId::HYB_GGA_XC_LRC_WPBE),
            474 => Ok(XcFuncId::HYB_MGGA_X_MVSH),
            475 => Ok(XcFuncId::HYB_GGA_XC_B3LYP5),
            476 => Ok(XcFuncId::HYB_GGA_XC_EDF2),
            477 => Ok(XcFuncId::HYB_GGA_XC_CAP0),
            478 => Ok(XcFuncId::HYB_GGA_XC_LC_WPBE),
            479 => Ok(XcFuncId::HYB_GGA_XC_HSE12),
            480 => Ok(XcFuncId::HYB_GGA_XC_HSE12S),
            481 => Ok(XcFuncId::HYB_GGA_XC_HSE_SOL),
            482 => Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_01),
            483 => Ok(XcFuncId::HYB_GGA_XC_MPW1LYP),
            484 => Ok(XcFuncId::HYB_GGA_XC_MPW1PBE),
            485 => Ok(XcFuncId::HYB_GGA_XC_KMLYP),
            486 => Ok(XcFuncId::HYB_GGA_XC_LC_WPBE_WHS),
            487 => Ok(XcFuncId::HYB_GGA_XC_LC_WPBEH_WHS),
            488 => Ok(XcFuncId::HYB_GGA_XC_LC_WPBE08_WHS),
            489 => Ok(XcFuncId::HYB_GGA_XC_LC_WPBESOL_WHS),
            490 => Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_00),
            491 => Ok(XcFuncId::HYB_GGA_XC_CAM_QTP_02),
            492 => Ok(XcFuncId::HYB_GGA_XC_LC_QTP),
            493 => Ok(XcFuncId::MGGA_X_RSCAN),
            494 => Ok(XcFuncId::MGGA_C_RSCAN),
            495 => Ok(XcFuncId::GGA_X_S12G),
            496 => Ok(XcFuncId::HYB_GGA_X_S12H),
            497 => Ok(XcFuncId::MGGA_X_R2SCAN),
            498 => Ok(XcFuncId::MGGA_C_R2SCAN),
            499 => Ok(XcFuncId::HYB_GGA_XC_BLYP35),
            500 => Ok(XcFuncId::GGA_K_VW),
            501 => Ok(XcFuncId::GGA_K_GE2),
            502 => Ok(XcFuncId::GGA_K_GOLDEN),
            503 => Ok(XcFuncId::GGA_K_YT65),
            504 => Ok(XcFuncId::GGA_K_BALTIN),
            505 => Ok(XcFuncId::GGA_K_LIEB),
            506 => Ok(XcFuncId::GGA_K_ABSP1),
            507 => Ok(XcFuncId::GGA_K_ABSP2),
            508 => Ok(XcFuncId::GGA_K_GR),
            509 => Ok(XcFuncId::GGA_K_LUDENA),
            510 => Ok(XcFuncId::GGA_K_GP85),
            511 => Ok(XcFuncId::GGA_K_PEARSON),
            512 => Ok(XcFuncId::GGA_K_OL1),
            513 => Ok(XcFuncId::GGA_K_OL2),
            514 => Ok(XcFuncId::GGA_K_FR_B88),
            515 => Ok(XcFuncId::GGA_K_FR_PW86),
            516 => Ok(XcFuncId::GGA_K_DK),
            517 => Ok(XcFuncId::GGA_K_PERDEW),
            518 => Ok(XcFuncId::GGA_K_VSK),
            519 => Ok(XcFuncId::GGA_K_VJKS),
            520 => Ok(XcFuncId::GGA_K_ERNZERHOF),
            521 => Ok(XcFuncId::GGA_K_LC94),
            522 => Ok(XcFuncId::GGA_K_LLP),
            523 => Ok(XcFuncId::GGA_K_THAKKAR),
            524 => Ok(XcFuncId::GGA_X_WPBEH),
            525 => Ok(XcFuncId::GGA_X_HJS_PBE),
            526 => Ok(XcFuncId::GGA_X_HJS_PBE_SOL),
            527 => Ok(XcFuncId::GGA_X_HJS_B88),
            528 => Ok(XcFuncId::GGA_X_HJS_B97X),
            529 => Ok(XcFuncId::GGA_X_ITYH),
            530 => Ok(XcFuncId::GGA_X_SFAT),
            531 => Ok(XcFuncId::HYB_MGGA_XC_WB97M_V),
            532 => Ok(XcFuncId::LDA_X_REL),
            533 => Ok(XcFuncId::GGA_X_SG4),
            534 => Ok(XcFuncId::GGA_C_SG4),
            535 => Ok(XcFuncId::GGA_X_GG99),
            536 => Ok(XcFuncId::LDA_XC_1D_EHWLRG_1),
            537 => Ok(XcFuncId::LDA_XC_1D_EHWLRG_2),
            538 => Ok(XcFuncId::LDA_XC_1D_EHWLRG_3),
            539 => Ok(XcFuncId::GGA_X_PBEPOW),
            540 => Ok(XcFuncId::MGGA_X_TM),
            541 => Ok(XcFuncId::MGGA_X_VT84),
            542 => Ok(XcFuncId::MGGA_X_SA_TPSS),
            543 => Ok(XcFuncId::MGGA_K_PC07),
            544 => Ok(XcFuncId::GGA_X_KGG99),
            545 => Ok(XcFuncId::GGA_XC_HLE16),
            546 => Ok(XcFuncId::LDA_X_ERF),
            547 => Ok(XcFuncId::LDA_XC_LP_A),
            548 => Ok(XcFuncId::LDA_XC_LP_B),
            549 => Ok(XcFuncId::LDA_X_RAE),
            550 => Ok(XcFuncId::LDA_K_ZLP),
            551 => Ok(XcFuncId::LDA_C_MCWEENY),
            552 => Ok(XcFuncId::LDA_C_BR78),
            553 => Ok(XcFuncId::GGA_C_SCAN_E0),
            554 => Ok(XcFuncId::LDA_C_PK09),
            555 => Ok(XcFuncId::GGA_C_GAPC),
            556 => Ok(XcFuncId::GGA_C_GAPLOC),
            557 => Ok(XcFuncId::GGA_C_ZVPBEINT),
            558 => Ok(XcFuncId::GGA_C_ZVPBESOL),
            559 => Ok(XcFuncId::GGA_C_TM_LYP),
            560 => Ok(XcFuncId::GGA_C_TM_PBE),
            561 => Ok(XcFuncId::GGA_C_W94),
            562 => Ok(XcFuncId::MGGA_C_KCIS),
            563 => Ok(XcFuncId::HYB_MGGA_XC_B0KCIS),
            564 => Ok(XcFuncId::MGGA_XC_LP90),
            565 => Ok(XcFuncId::GGA_C_CS1),
            566 => Ok(XcFuncId::HYB_MGGA_XC_MPW1KCIS),
            567 => Ok(XcFuncId::HYB_MGGA_XC_MPWKCIS1K),
            568 => Ok(XcFuncId::HYB_MGGA_XC_PBE1KCIS),
            569 => Ok(XcFuncId::HYB_MGGA_XC_TPSS1KCIS),
            570 => Ok(XcFuncId::GGA_X_B88M),
            571 => Ok(XcFuncId::MGGA_C_B88),
            572 => Ok(XcFuncId::HYB_GGA_XC_B5050LYP),
            573 => Ok(XcFuncId::LDA_C_OW_LYP),
            574 => Ok(XcFuncId::LDA_C_OW),
            575 => Ok(XcFuncId::MGGA_X_GX),
            576 => Ok(XcFuncId::MGGA_X_PBE_GX),
            577 => Ok(XcFuncId::LDA_XC_GDSMFB),
            578 => Ok(XcFuncId::LDA_C_GK72),
            579 => Ok(XcFuncId::LDA_C_KARASIEV),
            580 => Ok(XcFuncId::LDA_K_LP96),
            581 => Ok(XcFuncId::MGGA_X_REVSCAN),
            582 => Ok(XcFuncId::MGGA_C_REVSCAN),
            583 => Ok(XcFuncId::HYB_MGGA_X_REVSCAN0),
            584 => Ok(XcFuncId::MGGA_C_SCAN_VV10),
            585 => Ok(XcFuncId::MGGA_C_REVSCAN_VV10),
            586 => Ok(XcFuncId::MGGA_X_BR89_EXPLICIT),
            587 => Ok(XcFuncId::GGA_XC_KT3),
            588 => Ok(XcFuncId::HYB_LDA_XC_BN05),
            589 => Ok(XcFuncId::HYB_GGA_XC_LB07),
            590 => Ok(XcFuncId::LDA_C_PMGB06),
            591 => Ok(XcFuncId::GGA_K_GDS08),
            592 => Ok(XcFuncId::GGA_K_GHDS10),
            593 => Ok(XcFuncId::GGA_K_GHDS10R),
            594 => Ok(XcFuncId::GGA_K_TKVLN),
            595 => Ok(XcFuncId::GGA_K_PBE3),
            596 => Ok(XcFuncId::GGA_K_PBE4),
            597 => Ok(XcFuncId::GGA_K_EXP4),
            598 => Ok(XcFuncId::HYB_MGGA_XC_B98),
            599 => Ok(XcFuncId::LDA_XC_TIH),
            600 => Ok(XcFuncId::LDA_X_1D_EXPONENTIAL),
            601 => Ok(XcFuncId::GGA_X_SFAT_PBE),
            602 => Ok(XcFuncId::MGGA_X_BR89_EXPLICIT_1),
            603 => Ok(XcFuncId::MGGA_X_REGTPSS),
            604 => Ok(XcFuncId::GGA_X_FD_LB94),
            605 => Ok(XcFuncId::GGA_X_FD_REVLB94),
            606 => Ok(XcFuncId::GGA_C_ZVPBELOC),
            607 => Ok(XcFuncId::HYB_GGA_XC_APBE0),
            608 => Ok(XcFuncId::HYB_GGA_XC_HAPBE),
            609 => Ok(XcFuncId::MGGA_X_2D_JS17),
            610 => Ok(XcFuncId::HYB_GGA_XC_RCAM_B3LYP),
            611 => Ok(XcFuncId::HYB_GGA_XC_WC04),
            612 => Ok(XcFuncId::HYB_GGA_XC_WP04),
            613 => Ok(XcFuncId::GGA_K_LKT),
            614 => Ok(XcFuncId::HYB_GGA_XC_CAMH_B3LYP),
            615 => Ok(XcFuncId::HYB_GGA_XC_WHPBE0),
            616 => Ok(XcFuncId::GGA_K_PBE2),
            617 => Ok(XcFuncId::MGGA_K_L04),
            618 => Ok(XcFuncId::MGGA_K_L06),
            619 => Ok(XcFuncId::GGA_K_VT84F),
            620 => Ok(XcFuncId::GGA_K_LGAP),
            621 => Ok(XcFuncId::MGGA_K_RDA),
            622 => Ok(XcFuncId::GGA_X_ITYH_OPTX),
            623 => Ok(XcFuncId::GGA_X_ITYH_PBE),
            624 => Ok(XcFuncId::GGA_C_LYPR),
            625 => Ok(XcFuncId::HYB_GGA_XC_LC_BLYP_EA),
            626 => Ok(XcFuncId::MGGA_X_REGTM),
            627 => Ok(XcFuncId::MGGA_K_GEA2),
            628 => Ok(XcFuncId::MGGA_K_GEA4),
            629 => Ok(XcFuncId::MGGA_K_CSK1),
            630 => Ok(XcFuncId::MGGA_K_CSK4),
            631 => Ok(XcFuncId::MGGA_K_CSK_LOC1),
            632 => Ok(XcFuncId::MGGA_K_CSK_LOC4),
            633 => Ok(XcFuncId::GGA_K_LGAP_GE),
            634 => Ok(XcFuncId::MGGA_K_PC07_OPT),
            635 => Ok(XcFuncId::GGA_K_TFVW_OPT),
            636 => Ok(XcFuncId::HYB_GGA_XC_LC_BOP),
            637 => Ok(XcFuncId::HYB_GGA_XC_LC_PBEOP),
            638 => Ok(XcFuncId::MGGA_C_KCISK),
            639 => Ok(XcFuncId::HYB_GGA_XC_LC_BLYPR),
            640 => Ok(XcFuncId::HYB_GGA_XC_MCAM_B3LYP),
            641 => Ok(XcFuncId::LDA_X_YUKAWA),
            642 => Ok(XcFuncId::MGGA_C_R2SCAN01),
            643 => Ok(XcFuncId::MGGA_C_RMGGAC),
            644 => Ok(XcFuncId::MGGA_X_MCML),
            645 => Ok(XcFuncId::MGGA_X_R2SCAN01),
            646 => Ok(XcFuncId::HYB_GGA_X_CAM_S12G),
            647 => Ok(XcFuncId::HYB_GGA_X_CAM_S12H),
            648 => Ok(XcFuncId::MGGA_X_RPPSCAN),
            649 => Ok(XcFuncId::MGGA_C_RPPSCAN),
            650 => Ok(XcFuncId::MGGA_X_R4SCAN),
            651 => Ok(XcFuncId::MGGA_X_VCML),
            652 => Ok(XcFuncId::MGGA_XC_VCML_RVV10),
            653 => Ok(XcFuncId::HYB_LDA_X_ERF),
            654 => Ok(XcFuncId::LDA_C_PW_ERF),
            655 => Ok(XcFuncId::GGA_X_PBE_ERF_GWS),
            656 => Ok(XcFuncId::HYB_GGA_X_PBE_ERF_GWS),
            657 => Ok(XcFuncId::GGA_C_PBE_ERF_GWS),
            658 => Ok(XcFuncId::HYB_MGGA_XC_GAS22),
            659 => Ok(XcFuncId::HYB_MGGA_XC_R2SCANH),
            660 => Ok(XcFuncId::HYB_MGGA_XC_R2SCAN0),
            661 => Ok(XcFuncId::HYB_MGGA_XC_R2SCAN50),
            662 => Ok(XcFuncId::HYB_MGGA_X_WR2SCAN),
            681 => Ok(XcFuncId::HYB_GGA_XC_CAM_PBEH),
            682 => Ok(XcFuncId::HYB_GGA_XC_CAMY_PBEH),
            683 => Ok(XcFuncId::LDA_C_UPW92),
            684 => Ok(XcFuncId::LDA_C_RPW92),
            685 => Ok(XcFuncId::MGGA_X_TLDA),
            686 => Ok(XcFuncId::MGGA_X_EDMGGA),
            687 => Ok(XcFuncId::MGGA_X_GDME_NV),
            688 => Ok(XcFuncId::MGGA_X_RLDA),
            689 => Ok(XcFuncId::MGGA_X_GDME_0),
            690 => Ok(XcFuncId::MGGA_X_GDME_KOS),
            691 => Ok(XcFuncId::MGGA_X_GDME_VT),
            692 => Ok(XcFuncId::LDA_X_SLOC),
            693 => Ok(XcFuncId::MGGA_X_REVTM),
            694 => Ok(XcFuncId::MGGA_C_REVTM),
            695 => Ok(XcFuncId::HYB_MGGA_XC_EDMGGAH),
            696 => Ok(XcFuncId::MGGA_X_MBRXC_BG),
            697 => Ok(XcFuncId::MGGA_X_MBRXH_BG),
            698 => Ok(XcFuncId::MGGA_X_HLTA),
            699 => Ok(XcFuncId::MGGA_C_HLTAPW),
            700 => Ok(XcFuncId::MGGA_X_SCANL),
            701 => Ok(XcFuncId::MGGA_X_REVSCANL),
            702 => Ok(XcFuncId::MGGA_C_SCANL),
            703 => Ok(XcFuncId::MGGA_C_SCANL_RVV10),
            704 => Ok(XcFuncId::MGGA_C_SCANL_VV10),
            705 => Ok(XcFuncId::HYB_MGGA_X_JS18),
            706 => Ok(XcFuncId::HYB_MGGA_X_PJS18),
            707 => Ok(XcFuncId::MGGA_X_TASK),
            711 => Ok(XcFuncId::MGGA_X_MGGAC),
            712 => Ok(XcFuncId::GGA_C_MGGAC),
            716 => Ok(XcFuncId::MGGA_X_MBR),
            718 => Ok(XcFuncId::MGGA_X_R2SCANL),
            719 => Ok(XcFuncId::MGGA_C_R2SCANL),
            720 => Ok(XcFuncId::HYB_MGGA_XC_LC_TMLYP),
            724 => Ok(XcFuncId::MGGA_X_MTASK),
            734 => Ok(XcFuncId::GGA_X_Q1D),
            735 => Ok(XcFuncId::MGGA_X_KTBM_0),
            736 => Ok(XcFuncId::MGGA_X_KTBM_1),
            737 => Ok(XcFuncId::MGGA_X_KTBM_2),
            738 => Ok(XcFuncId::MGGA_X_KTBM_3),
            739 => Ok(XcFuncId::MGGA_X_KTBM_4),
            740 => Ok(XcFuncId::MGGA_X_KTBM_5),
            741 => Ok(XcFuncId::MGGA_X_KTBM_6),
            742 => Ok(XcFuncId::MGGA_X_KTBM_7),
            743 => Ok(XcFuncId::MGGA_X_KTBM_8),
            744 => Ok(XcFuncId::MGGA_X_KTBM_9),
            745 => Ok(XcFuncId::MGGA_X_KTBM_10),
            746 => Ok(XcFuncId::MGGA_X_KTBM_11),
            747 => Ok(XcFuncId::MGGA_X_KTBM_12),
            748 => Ok(XcFuncId::MGGA_X_KTBM_13),
            749 => Ok(XcFuncId::MGGA_X_KTBM_14),
            750 => Ok(XcFuncId::MGGA_X_KTBM_15),
            751 => Ok(XcFuncId::MGGA_X_KTBM_16),
            752 => Ok(XcFuncId::MGGA_X_KTBM_17),
            753 => Ok(XcFuncId::MGGA_X_KTBM_18),
            754 => Ok(XcFuncId::MGGA_X_KTBM_19),
            755 => Ok(XcFuncId::MGGA_X_KTBM_20),
            756 => Ok(XcFuncId::MGGA_X_KTBM_21),
            757 => Ok(XcFuncId::MGGA_X_KTBM_22),
            758 => Ok(XcFuncId::MGGA_X_KTBM_23),
            759 => Ok(XcFuncId::MGGA_X_KTBM_24),
            760 => Ok(XcFuncId::MGGA_X_KTBM_GAP),
            761 => Ok(XcFuncId::MGGA_X_MSPBEL),
            762 => Ok(XcFuncId::MGGA_X_RMSPBEL),
            763 => Ok(XcFuncId::MGGA_X_MSRPBEL),
            764 => Ok(XcFuncId::MGGA_X_RMSRPBEL),
            765 => Ok(XcFuncId::MGGA_X_MSB86BL),
            766 => Ok(XcFuncId::MGGA_X_RMSB86BL),
            767 => Ok(XcFuncId::HYB_MGGA_X_PI_M06_2X_DL),
            768 => Ok(XcFuncId::MGGA_C_PI_M06_2X_DL),
            769 => Ok(XcFuncId::HYB_MGGA_X_PI_M06_2X),
            770 => Ok(XcFuncId::MGGA_C_PI_M06_2X),
            _ => Err(format!("Unknown libxc functional number: {value}")),
        }
    }
}
