#[doc = "Register `RST1` reader"]
pub type R = crate::R<Rst1Spec>;
#[doc = "Register `RST1` writer"]
pub type W = crate::W<Rst1Spec>;
#[doc = "Field `DMAC1_RST_N` reader - Dmac1 rst n"]
pub type Dmac1RstNR = crate::BitReader;
#[doc = "Field `DMAC1_RST_N` writer - Dmac1 rst n"]
pub type Dmac1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC0_RST_N` reader - Dmac0 rst n"]
pub type Dmac0RstNR = crate::BitReader;
#[doc = "Field `DMAC0_RST_N` writer - Dmac0 rst n"]
pub type Dmac0RstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_RST_N` reader - I2s rst n"]
pub type I2sRstNR = crate::BitReader;
#[doc = "Field `I2S_RST_N` writer - I2s rst n"]
pub type I2sRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGC_RST_N` reader - Rngc rst n"]
pub type RngcRstNR = crate::BitReader;
#[doc = "Field `RNGC_RST_N` writer - Rngc rst n"]
pub type RngcRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMER1_RST_N` reader - Lptimer1 rst n"]
pub type Lptimer1RstNR = crate::BitReader;
#[doc = "Field `LPTIMER1_RST_N` writer - Lptimer1 rst n"]
pub type Lptimer1RstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dmac1 rst n"]
    #[inline(always)]
    pub fn dmac1_rst_n(&self) -> Dmac1RstNR {
        Dmac1RstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dmac0 rst n"]
    #[inline(always)]
    pub fn dmac0_rst_n(&self) -> Dmac0RstNR {
        Dmac0RstNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2s rst n"]
    #[inline(always)]
    pub fn i2s_rst_n(&self) -> I2sRstNR {
        I2sRstNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rngc rst n"]
    #[inline(always)]
    pub fn rngc_rst_n(&self) -> RngcRstNR {
        RngcRstNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lptimer1 rst n"]
    #[inline(always)]
    pub fn lptimer1_rst_n(&self) -> Lptimer1RstNR {
        Lptimer1RstNR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dmac1 rst n"]
    #[inline(always)]
    pub fn dmac1_rst_n(&mut self) -> Dmac1RstNW<'_, Rst1Spec> {
        Dmac1RstNW::new(self, 0)
    }
    #[doc = "Bit 1 - Dmac0 rst n"]
    #[inline(always)]
    pub fn dmac0_rst_n(&mut self) -> Dmac0RstNW<'_, Rst1Spec> {
        Dmac0RstNW::new(self, 1)
    }
    #[doc = "Bit 2 - I2s rst n"]
    #[inline(always)]
    pub fn i2s_rst_n(&mut self) -> I2sRstNW<'_, Rst1Spec> {
        I2sRstNW::new(self, 2)
    }
    #[doc = "Bit 3 - Rngc rst n"]
    #[inline(always)]
    pub fn rngc_rst_n(&mut self) -> RngcRstNW<'_, Rst1Spec> {
        RngcRstNW::new(self, 3)
    }
    #[doc = "Bit 4 - Lptimer1 rst n"]
    #[inline(always)]
    pub fn lptimer1_rst_n(&mut self) -> Lptimer1RstNW<'_, Rst1Spec> {
        Lptimer1RstNW::new(self, 4)
    }
}
#[doc = "reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rst1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst1Spec;
impl crate::RegisterSpec for Rst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for Rst1Spec {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for Rst1Spec {
    type Safety = crate::Unsafe;
}
