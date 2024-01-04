#![allow(unused)]

pub  const FAMILY_ID: u16 = 0x8994;

pub  const AIF1_CONTROL1_ADCR_SRC_MASK: u16 = 0x4000;
pub  const AIF1_CONTROL1_ADCR_SRC_POSITION: u16 = 14;

pub  const AIF1_CONTROL1_WL_MASK: u16 = 0x0060;
pub  const AIF1_CONTROL1_WL_POSITION: u16 = 5;

pub  const SW_RESET:u16 =0x0000;

/* Power Management */
pub  const PWR_MANAGEMENT_1:u16 =0x0001;
pub  const PWR_MANAGEMENT_2:u16 =0x0002;
pub  const PWR_MANAGEMENT_3:u16 =0x0003;
pub  const PWR_MANAGEMENT_4:u16 =0x0004;
pub  const PWR_MANAGEMENT_5:u16 =0x0005;
pub  const PWR_MANAGEMENT_6:u16 =0x0006;

/* Input mixer */
pub  const INPUT_MIXER_1:u16 =0x0015;
/* Input volume */
pub  const LEFT_LINE_IN12_VOL:u16 =0x0018;
pub  const LEFT_LINE_IN34_VOL:u16 =0x0019;
pub  const RIGHT_LINE_IN12_VOL:u16 =0x001A;
pub  const RIGHT_LINE_IN34_VOL:u16 =0x001B;

/* L/R Output volumes */
pub  const LEFT_OUTPUT_VOL:u16 =0x001C;
pub  const RIGHT_OUTPUT_VOL:u16 =0x001D;
pub  const LINE_OUTPUT_VOL:u16 =0x001E;
pub  const OUTPUT2_VOL:u16 =0x001F;


/* L/R OPGA volumes */
pub  const LEFT_OPGA_VOL:u16 =0x0020;
pub  const RIGHT_OPGA_VOL:u16 =0x0021;

/* SPKMIXL/R Attenuation */
pub  const SPKMIXL_ATT:u16 =0x0022;
pub  const SPKMIXR_ATT:u16 =0x0023;
pub  const OUTPUT_MIXER:u16 =0x0024;
pub  const CLASS_D:u16 =0x0025;
/* L/R Speakers volumes */
pub  const SPK_LEFT_VOL:u16 =0x0026;
pub  const SPK_RIGHT_VOL:u16 =0x0027;

/* Input mixer */
pub  const INPUT_MIXER_2:u16 =0x0028;
pub  const INPUT_MIXER_3:u16 =0x0029;
pub  const INPUT_MIXER_4:u16 =0x002A;
pub  const INPUT_MIXER_5:u16 =0x002B;
pub  const INPUT_MIXER_6:u16 =0x002C;

/* Output mixer */
pub  const OUTPUT_MIXER_1:u16 =0x002D;
pub  const OUTPUT_MIXER_2:u16 =0x002E;
pub  const OUTPUT_MIXER_3:u16 =0x002F;
pub  const OUTPUT_MIXER_4:u16 =0x0030;
pub  const OUTPUT_MIXER_5:u16 =0x0031;
pub  const OUTPUT_MIXER_6:u16 =0x0032;
pub  const OUTPUT2_MIXER:u16 =0x0033;
pub  const LINE_MIXER_1:u16 =0x0034;
pub  const LINE_MIXER_2:u16 =0x0035;
pub  const SPEAKER_MIXER:u16 =0x0036;
pub  const ADD_CONTROL:u16 =0x0037;
/* Antipop */
pub  const ANTIPOP1:u16 =0x0038;
pub  const ANTIPOP2:u16 =0x0039;
pub  const MICBIAS:u16 =0x003A;
pub  const LDO1:u16 =0x003B;
pub  const LDO2:u16 =0x003C;

/* Charge pump */
pub  const CHARGE_PUMP1:u16 =0x004C;
pub  const CHARGE_PUMP2:u16 =0x004D;

pub  const CLASS_W:u16 =0x0051;

pub  const DC_SERVO1:u16 =0x0054;
pub  const DC_SERVO2:u16 =0x0055;
pub  const DC_SERVO_READBACK:u16 =0x0058;
pub  const DC_SERVO_WRITEVAL:u16 =0x0059;

/* Analog HP */
pub  const ANALOG_HP:u16 =0x0060;

pub  const CHIP_REVISION:u16 =0x0100;
pub  const CONTROL_INTERFACE:u16 =0x0101;
pub  const WRITE_SEQ_CTRL1:u16 =0x0110;
pub  const WRITE_SEQ_CTRL2:u16 =0x0111;

/* WM8994 clocking */
pub  const AIF1_CLOCKING1:u16 =0x0200;
pub  const AIF1_CLOCKING2:u16 =0x0201;
pub  const AIF2_CLOCKING1:u16 =0x0204;
pub  const AIF2_CLOCKING2:u16 =0x0205;
pub  const CLOCKING1:u16 =0x0208;
pub  const CLOCKING2:u16 =0x0209;
pub  const AIF1_RATE:u16 =0x0210;
pub  const AIF2_RATE:u16 =0x0211;
pub  const RATE_STATUS:u16 =0x0212;

/* FLL1 Control */
pub  const FLL1_CONTROL1:u16 =0x0220;
pub  const FLL1_CONTROL2:u16 =0x0221;
pub  const FLL1_CONTROL3:u16 =0x0222;
pub  const FLL1_CONTROL4:u16 =0x0223;
pub  const FLL1_CONTROL5:u16 =0x0224;

/* FLL2 Control */
pub  const FLL2_CONTROL1:u16 =0x0240;
pub  const FLL2_CONTROL2:u16 =0x0241;
pub  const FLL2_CONTROL3:u16 =0x0242;
pub  const FLL2_CONTROL4:u16 =0x0243;
pub  const FLL2_CONTROL5:u16 =0x0244;


/* AIF1 control */
pub  const AIF1_CONTROL1:u16 =0x0300;
pub  const AIF1_CONTROL2:u16 =0x0301;
pub  const AIF1_MASTER_SLAVE:u16 =0x0302;
pub  const AIF1_BCLK:u16 =0x0303;
pub  const AIF1_ADC_LRCLK:u16 =0x0304;
pub  const AIF1_DAC_LRCLK:u16 =0x0305;
pub  const AIF1_DAC_DELTA:u16 =0x0306;
pub  const AIF1_ADC_DELTA:u16 =0x0307;

/* AIF2 control */
pub  const AIF2_CONTROL1:u16 =0x0310;
pub  const AIF2_CONTROL2:u16 =0x0311;
pub  const AIF2_MASTER_SLAVE:u16 =0x0312;
pub  const AIF2_BCLK:u16 =0x0313;
pub  const AIF2_ADC_LRCLK:u16 =0x0314;
pub  const AIF2_DAC_LRCLK:u16 =0x0315;
pub  const AIF2_DAC_DELTA:u16 =0x0316;
pub  const AIF2_ADC_DELTA:u16 =0x0317;

/* AIF1 ADC/DAC LR volumes */
pub  const AIF1_ADC1_LEFT_VOL:u16 =0x0400;
pub  const AIF1_ADC1_RIGHT_VOL:u16 =0x0401;
pub  const AIF1_DAC1_LEFT_VOL:u16 =0x0402;
pub  const AIF1_DAC1_RIGHT_VOL:u16 =0x0403;
pub  const AIF1_ADC2_LEFT_VOL:u16 =0x0404;
pub  const AIF1_ADC2_RIGHT_VOL:u16 =0x0405;
pub  const AIF1_DAC2_LEFT_VOL:u16 =0x0406;
pub  const AIF1_DAC2_RIGHT_VOL:u16 =0x0407;

/* AIF1 ADC/DAC filters */
pub  const AIF1_ADC1_FILTERS:u16 =0x0410;
pub  const AIF1_ADC2_FILTERS:u16 =0x0411;
pub  const AIF1_DAC1_FILTER1:u16 =0x0420;
pub  const AIF1_DAC1_FILTER2:u16 =0x0421;
pub  const AIF1_DAC2_FILTER1:u16 =0x0422;
pub  const AIF1_DAC2_FILTER2:u16 =0x0423;

/* AIF1 DRC1 registers */
pub  const AIF1_DRC1:u16 =0x0440;
pub  const AIF1_DRC1_1:u16 =0x0441;
pub  const AIF1_DRC1_2:u16 =0x0442;
pub  const AIF1_DRC1_3:u16 =0x0443;
pub  const AIF1_DRC1_4:u16 =0x0444;
/* AIF1 DRC2 registers */
pub  const AIF1_DRC2:u16 =0x0450;
pub  const AIF1_DRC2_1:u16 =0x0451;
pub  const AIF1_DRC2_2:u16 =0x0452;
pub  const AIF1_DRC2_3:u16 =0x0453;
pub  const AIF1_DRC2_4:u16 =0x0454;

/* AIF1 DAC1 EQ Gains Bands */
pub  const AIF1_DAC1_EQG_1:u16 =0x0480;
pub  const AIF1_DAC1_EQG_2:u16 =0x0481;
pub  const AIF1_DAC1_EQG_1A:u16 =0x0482;
pub  const AIF1_DAC1_EQG_1B:u16 =0x0483;
pub  const AIF1_DAC1_EQG_1PG:u16 =0x0484;
pub  const AIF1_DAC1_EQG_2A:u16 =0x0485;
pub  const AIF1_DAC1_EQG_2B:u16 =0x0486;
pub  const AIF1_DAC1_EQG_2C:u16 =0x0487;
pub  const AIF1_DAC1_EQG_2PG:u16 =0x0488;
pub  const AIF1_DAC1_EQG_3A:u16 =0x0489;
pub  const AIF1_DAC1_EQG_3B:u16 =0x048A;
pub  const AIF1_DAC1_EQG_3C:u16 =0x048B;
pub  const AIF1_DAC1_EQG_3PG:u16 =0x048C;
pub  const AIF1_DAC1_EQG_4A:u16 =0x048D;
pub  const AIF1_DAC1_EQG_4B:u16 =0x048E;
pub  const AIF1_DAC1_EQG_4C:u16 =0x048F;
pub  const AIF1_DAC1_EQG_4PG:u16 =0x0490;
pub  const AIF1_DAC1_EQG_5A:u16 =0x0491;
pub  const AIF1_DAC1_EQG_5B:u16 =0x0492;
pub  const AIF1_DAC1_EQG_5PG:u16 =0x0493;

/* AIF1 DAC2 EQ Gains/bands */
pub  const AIF1_DAC2_EQG_1:u16 =0x04A0;
pub  const AIF1_DAC2_EQG_2:u16 =0x04A1;
pub  const AIF1_DAC2_EQG_1A:u16 =0x04A2;
pub  const AIF1_DAC2_EQG_1B:u16 =0x04A3;
pub  const AIF1_DAC2_EQG_1PG:u16 =0x04A4;
pub  const AIF1_DAC2_EQG_2A:u16 =0x04A5;
pub  const AIF1_DAC2_EQG_2B:u16 =0x04A6;
pub  const AIF1_DAC2_EQG_2C:u16 =0x04A7;
pub  const AIF1_DAC2_EQG_2PG:u16 =0x04A8;
pub  const AIF1_DAC2_EQG_3A:u16 =0x04A9;
pub  const AIF1_DAC2_EQG_3B:u16 =0x04AA;
pub  const AIF1_DAC2_EQG_3C:u16 =0x04AB;
pub  const AIF1_DAC2_EQG_3PG:u16 =0x04AC;
pub  const AIF1_DAC2_EQG_4A:u16 =0x04AD;
pub  const AIF1_DAC2_EQG_4B:u16 =0x04AE;
pub  const AIF1_DAC2_EQG_4C:u16 =0x04AF;
pub  const AIF1_DAC2_EQG_4PG:u16 =0x04B0;
pub  const AIF1_DAC2_EQG_5A:u16 =0x04B1;
pub  const AIF1_DAC2_EQG_5B:u16 =0x04B2;
pub  const AIF1_DAC2_EQG_5PG:u16 =0x04B3;

/* AIF2 ADC/DAC LR volumes */
pub  const AIF2_ADC_LEFT_VOL:u16 =0x0500;
pub  const AIF2_ADC_RIGHT_VOL:u16 =0x0501;
pub  const AIF2_DAC_LEFT_VOL:u16 =0x0502;
pub  const AIF2_DAC_RIGHT_VOL:u16 =0x0503;

/* AIF2 ADC/DAC filters */
pub  const AIF2_ADC_FILTERS:u16 =0x0510;
pub  const AIF2_DAC_FILTER_1:u16 =0x0520;
pub  const AIF2_DAC_FILTER_2:u16 =0x0521;

/* AIF2 DRC registers */
pub  const AIF2_DRC_1:u16 =0x0540;
pub  const AIF2_DRC_2:u16 =0x0541;
pub  const AIF2_DRC_3:u16 =0x0542;
pub  const AIF2_DRC_4:u16 =0x0543;
pub  const AIF2_DRC_5:u16 =0x0544;

/* AIF2 EQ Gains/bands */
pub  const AIF2_EQG_1:u16 =0x0580;
pub  const AIF2_EQG_2:u16 =0x0581;
pub  const AIF2_EQG_1A:u16 =0x0582;
pub  const AIF2_EQG_1B:u16 =0x0583;
pub  const AIF2_EQG_1PG:u16 =0x0584;
pub  const AIF2_EQG_2A:u16 =0x0585;
pub  const AIF2_EQG_2B:u16 =0x0586;
pub  const AIF2_EQG_2C:u16 =0x0587;
pub  const AIF2_EQG_2PG:u16 =0x0588;
pub  const AIF2_EQG_3A:u16 =0x0589;
pub  const AIF2_EQG_3B:u16 =0x058A;
pub  const AIF2_EQG_3C:u16 =0x058B;
pub  const AIF2_EQG_3PG:u16 =0x058C;
pub  const AIF2_EQG_4A:u16 =0x058D;
pub  const AIF2_EQG_4B:u16 =0x058E;
pub  const AIF2_EQG_4C:u16 =0x058F;
pub  const AIF2_EQG_4PG:u16 =0x0590;
pub  const AIF2_EQG_5A:u16 =0x0591;
pub  const AIF2_EQG_5B:u16 =0x0592;
pub  const AIF2_EQG_5PG:u16 =0x0593;

/* AIF1 DAC1 Mixer volume */
pub  const DAC1_MIXER_VOL:u16 =0x0600;
/* AIF1 DAC1 Left Mixer Routing */
pub  const AIF1_DAC1_LMR:u16 =0x0601;
/* AIF1 DAC1 Righ Mixer Routing */
pub  const AIF1_DAC1_RMR:u16 =0x0602;
/* AIF1 DAC2 Mixer volume */
pub  const DAC2_MIXER_VOL:u16 =0x0603;
/* AIF1 DAC2 Left Mixer Routing */
pub  const AIF1_DAC2_LMR:u16 =0x0604;
/* AIF1 DAC2 Righ Mixer Routing */
pub  const AIF1_DAC2_RMR:u16 =0x0605;
/* AIF1 ADC1 Left Mixer Routing */
pub  const AIF1_ADC1_LMR:u16 =0x0606;
/* AIF1 ADC1 Righ Mixer Routing */
pub  const AIF1_ADC1_RMR:u16 =0x0607;
/* AIF1 ADC2 Left Mixer Routing */
pub  const AIF1_ADC2_LMR:u16 =0x0608;
/* AIF1 ADC2 Righ Mixer Routing */
pub  const AIF1_ADC2_RMR:u16 =0x0609;

/* Volume control */
pub  const DAC1_LEFT_VOL:u16 =0x0610;
pub  const DAC1_RIGHT_VOL:u16 =0x0611;
pub  const DAC2_LEFT_VOL:u16 =0x0612;
pub  const DAC2_RIGHT_VOL:u16 =0x0613;
pub  const DAC_SOFTMUTE:u16 =0x0614;

pub  const OVERSAMPLING:u16 =0x0620;
pub  const SIDETONE:u16 =0x0621;

/* GPIO */
pub  const GPIO1:u16 =0x0700;
pub  const GPIO2:u16 =0x0701;
pub  const GPIO3:u16 =0x0702;
pub  const GPIO4:u16 =0x0703;
pub  const GPIO5:u16 =0x0704;
pub  const GPIO6:u16 =0x0705;
pub  const GPIO7:u16 =0x0706;
pub  const GPIO8:u16 =0x0707;
pub  const GPIO9:u16 =0x0708;
pub  const GPIO10:u16 =0x0709;
pub  const GPIO11:u16 =0x070A;
/* Pull Contol */
pub  const PULL_CONTROL_1:u16 =0x0720;
pub  const PULL_CONTROL_2:u16 =0x0721;
/* WM8994 Inturrupts */
pub  const INT_STATUS_1:u16 =0x0730;
pub  const INT_STATUS_2:u16 =0x0731;
pub  const INT_RAW_STATUS_2:u16 =0x0732;
pub  const INT_STATUS1_MASK:u16 =0x0738;
pub  const INT_STATUS2_MASK:u16 =0x0739;
pub  const INT_CONTROL:u16 =0x0740;
pub  const IRQ_DEBOUNCE:u16 =0x0748;

/* Write Sequencer registers from 0 to 511 */
pub  const WRITE_SEQUENCER0:u16 =0x3000;
pub  const WRITE_SEQUENCER1:u16 =0x3001;
pub  const WRITE_SEQUENCER2:u16 =0x3002;
pub  const WRITE_SEQUENCER3:u16 =0x3003;

pub  const WRITE_SEQUENCER4:u16 =0x3508;
pub  const WRITE_SEQUENCER5:u16 =0x3509;
pub  const WRITE_SEQUENCER6:u16 =0x3510;
pub  const WRITE_SEQUENCER7:u16 =0x3511;
