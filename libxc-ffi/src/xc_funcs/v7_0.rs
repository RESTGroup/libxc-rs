//! Libxc functional IDs for API version 7.0.
//!
//! This file is generated automatically from `xc_funcs_v7.0.h`.
//! Do not edit manually.

#![allow(non_camel_case_types)]
#![allow(clippy::enum_clike_unportable_variant)]

/// Libxc functional identifier numbers.
///
/// Each variant corresponds to a `XC_*` define in the libxc headers.
/// The numeric values match the C library exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Gunnarson & Lundqvist
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
    LDA_C_1D_CSC = 18,
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
    GGA_XC_TH_FL = 196,
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
    /// Exchange for accurate virtual orbital energies
    MGGA_X_MK00 = 230,
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
    /// Type-I band gap functional by Bhattacharjee, Koshi and Lee
    GGA_X_BKL1 = 338,
    /// Type-II band gap functional by Bhattacharjee, Koshi and Lee
    GGA_X_BKL2 = 339,
    /// Minnesota CF22D hybrid exchange functional
    HYB_MGGA_X_CF22D = 340,
    /// Minnesota CF22D correlation functional
    MGGA_C_CF22D = 341,
    /// Lebeda-Aschebrock-Kummel meta-GGA exchange
    MGGA_X_LAK = 342,
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
    /// BHandH i.e. BHLYP
    HYB_GGA_XC_BHANDH = 435,
    /// BHandHLYP
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
    /// Takkar and McCarthy reparametrization, also known as reLYP
    GGA_C_TM_LYP = 559,
    /// Thakkar and McCarthy reparametrization
    GGA_C_TM_PBE = 560,
    /// Wilson 94 (Eq. 25)
    GGA_C_W94 = 561,
    /// Krieger, Chen, Iafrate, and Savin
    MGGA_C_KCIS = 562,
    /// Hybrid based on KCIS
    HYB_MGGA_XC_B0KCIS = 563,
    /// Lee & Parr, Eq. (56)
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
}
