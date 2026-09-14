#[doc = "Register `CR10` reader"]
pub type R = crate::R<Cr10Spec>;
#[doc = "Register `CR10` writer"]
pub type W = crate::W<Cr10Spec>;
#[doc = "Field `QSPI_REMAP_SIZE` reader - address space for qspi remapping, aligned in 1kb"]
pub type QspiRemapSizeR = crate::FieldReader<u16>;
#[doc = "Field `QSPI_REMAP_SIZE` writer - address space for qspi remapping, aligned in 1kb"]
pub type QspiRemapSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "i2s works in master or slave mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2sModeSel {
    #[doc = "0: slave mode"]
    Slave = 0,
    #[doc = "1: master mode"]
    Master = 1,
}
impl From<I2sModeSel> for bool {
    #[inline(always)]
    fn from(variant: I2sModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S_MODE_SEL` reader - i2s works in master or slave mode"]
pub type I2sModeSelR = crate::BitReader<I2sModeSel>;
impl I2sModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2sModeSel {
        match self.bits {
            false => I2sModeSel::Slave,
            true => I2sModeSel::Master,
        }
    }
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == I2sModeSel::Slave
    }
    #[doc = "master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == I2sModeSel::Master
    }
}
#[doc = "Field `I2S_MODE_SEL` writer - i2s works in master or slave mode"]
pub type I2sModeSelW<'a, REG> = crate::BitWriter<'a, REG, I2sModeSel>;
impl<'a, REG> I2sModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(I2sModeSel::Slave)
    }
    #[doc = "master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(I2sModeSel::Master)
    }
}
#[doc = "Field `I2S_WS_LEN` reader - i2s main interface resolution configuration"]
pub type I2sWsLenR = crate::FieldReader;
#[doc = "Field `I2S_WS_LEN` writer - i2s main interface resolution configuration"]
pub type I2sWsLenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `I2S_WS_EN` reader - i2s ws enable"]
pub type I2sWsEnR = crate::BitReader;
#[doc = "Field `I2S_WS_EN` writer - i2s ws enable"]
pub type I2sWsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_WS_SEL` reader - i2s ws output delay enable"]
pub type I2sWsSelR = crate::BitReader;
#[doc = "Field `I2S_WS_SEL` writer - i2s ws output delay enable"]
pub type I2sWsSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - address space for qspi remapping, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_size(&self) -> QspiRemapSizeR {
        QspiRemapSizeR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - i2s works in master or slave mode"]
    #[inline(always)]
    pub fn i2s_mode_sel(&self) -> I2sModeSelR {
        I2sModeSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:21 - i2s main interface resolution configuration"]
    #[inline(always)]
    pub fn i2s_ws_len(&self) -> I2sWsLenR {
        I2sWsLenR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - i2s ws enable"]
    #[inline(always)]
    pub fn i2s_ws_en(&self) -> I2sWsEnR {
        I2sWsEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - i2s ws output delay enable"]
    #[inline(always)]
    pub fn i2s_ws_sel(&self) -> I2sWsSelR {
        I2sWsSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - address space for qspi remapping, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_size(&mut self) -> QspiRemapSizeW<'_, Cr10Spec> {
        QspiRemapSizeW::new(self, 0)
    }
    #[doc = "Bit 14 - i2s works in master or slave mode"]
    #[inline(always)]
    pub fn i2s_mode_sel(&mut self) -> I2sModeSelW<'_, Cr10Spec> {
        I2sModeSelW::new(self, 14)
    }
    #[doc = "Bits 15:21 - i2s main interface resolution configuration"]
    #[inline(always)]
    pub fn i2s_ws_len(&mut self) -> I2sWsLenW<'_, Cr10Spec> {
        I2sWsLenW::new(self, 15)
    }
    #[doc = "Bit 22 - i2s ws enable"]
    #[inline(always)]
    pub fn i2s_ws_en(&mut self) -> I2sWsEnW<'_, Cr10Spec> {
        I2sWsEnW::new(self, 22)
    }
    #[doc = "Bit 23 - i2s ws output delay enable"]
    #[inline(always)]
    pub fn i2s_ws_sel(&mut self) -> I2sWsSelW<'_, Cr10Spec> {
        I2sWsSelW::new(self, 23)
    }
}
#[doc = "control register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`cr10::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr10Spec;
impl crate::RegisterSpec for Cr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr10::R`](R) reader structure"]
impl crate::Readable for Cr10Spec {}
#[doc = "`write(|w| ..)` method takes [`cr10::W`](W) writer structure"]
impl crate::Writable for Cr10Spec {
    type Safety = crate::Unsafe;
}
