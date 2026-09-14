#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `PWRTCXO_EN_BAT` reader - Pwr tcxo enable control"]
pub type PwrtcxoEnBatR = crate::BitReader;
#[doc = "Field `PWRTCXO_EN_BAT` writer - Pwr tcxo enable control"]
pub type PwrtcxoEnBatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCXO_EN_BAT` reader - Tcxo enable control"]
pub type TcxoEnBatR = crate::BitReader;
#[doc = "Field `TCXO_EN_BAT` writer - Tcxo enable control"]
pub type TcxoEnBatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_32M_EN_BAT` reader - Clk 32m enable control"]
pub type Clk32mEnBatR = crate::BitReader;
#[doc = "Field `CLK_32M_EN_BAT` writer - Clk 32m enable control"]
pub type Clk32mEnBatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRESET_BAT` reader - Nreset bat control"]
pub type NresetBatR = crate::BitReader;
#[doc = "Field `NRESET_BAT` writer - Nreset bat control"]
pub type NresetBatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_BAT` reader - Por bat control"]
pub type PorBatR = crate::BitReader;
#[doc = "Field `POR_BAT` writer - Por bat control"]
pub type PorBatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pwr tcxo enable control"]
    #[inline(always)]
    pub fn pwrtcxo_en_bat(&self) -> PwrtcxoEnBatR {
        PwrtcxoEnBatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tcxo enable control"]
    #[inline(always)]
    pub fn tcxo_en_bat(&self) -> TcxoEnBatR {
        TcxoEnBatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clk 32m enable control"]
    #[inline(always)]
    pub fn clk_32m_en_bat(&self) -> Clk32mEnBatR {
        Clk32mEnBatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Nreset bat control"]
    #[inline(always)]
    pub fn nreset_bat(&self) -> NresetBatR {
        NresetBatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Por bat control"]
    #[inline(always)]
    pub fn por_bat(&self) -> PorBatR {
        PorBatR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pwr tcxo enable control"]
    #[inline(always)]
    pub fn pwrtcxo_en_bat(&mut self) -> PwrtcxoEnBatW<'_, Cr1Spec> {
        PwrtcxoEnBatW::new(self, 0)
    }
    #[doc = "Bit 1 - Tcxo enable control"]
    #[inline(always)]
    pub fn tcxo_en_bat(&mut self) -> TcxoEnBatW<'_, Cr1Spec> {
        TcxoEnBatW::new(self, 1)
    }
    #[doc = "Bit 2 - Clk 32m enable control"]
    #[inline(always)]
    pub fn clk_32m_en_bat(&mut self) -> Clk32mEnBatW<'_, Cr1Spec> {
        Clk32mEnBatW::new(self, 2)
    }
    #[doc = "Bit 5 - Nreset bat control"]
    #[inline(always)]
    pub fn nreset_bat(&mut self) -> NresetBatW<'_, Cr1Spec> {
        NresetBatW::new(self, 5)
    }
    #[doc = "Bit 7 - Por bat control"]
    #[inline(always)]
    pub fn por_bat(&mut self) -> PorBatW<'_, Cr1Spec> {
        PorBatW::new(self, 7)
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
