#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LBM` reader - loopback mode"]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - loopback mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - ssp enable"]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - ssp enable"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "master/slave mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "0: master mode"]
    Master = 0,
    #[doc = "1: slave mode"]
    Slave = 1,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - master/slave mode selection"]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            false => Ms::Master,
            true => Ms::Slave,
        }
    }
    #[doc = "master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ms::Master
    }
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ms::Slave
    }
}
#[doc = "Field `MS` writer - master/slave mode selection"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Master)
    }
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Slave)
    }
}
#[doc = "Field `SOD` reader - ssp output disable in slave mode"]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - ssp output disable in slave mode"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - loopback mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ssp enable"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - master/slave mode selection"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ssp output disable in slave mode"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - loopback mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Cr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - ssp enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<'_, Cr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - master/slave mode selection"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Cr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - ssp output disable in slave mode"]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<'_, Cr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
