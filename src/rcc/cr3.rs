#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `I2S_SCLK_DIV` reader - I2s sclk div"]
pub type I2sSclkDivR = crate::FieldReader;
#[doc = "Field `I2S_SCLK_DIV` writer - I2s sclk div"]
pub type I2sSclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S_MCLK_DIV` reader - I2s mclk div"]
pub type I2sMclkDivR = crate::FieldReader;
#[doc = "Field `I2S_MCLK_DIV` writer - I2s mclk div"]
pub type I2sMclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2s sclk div"]
    #[inline(always)]
    pub fn i2s_sclk_div(&self) -> I2sSclkDivR {
        I2sSclkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2s mclk div"]
    #[inline(always)]
    pub fn i2s_mclk_div(&self) -> I2sMclkDivR {
        I2sMclkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2s sclk div"]
    #[inline(always)]
    pub fn i2s_sclk_div(&mut self) -> I2sSclkDivW<'_, Cr3Spec> {
        I2sSclkDivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - I2s mclk div"]
    #[inline(always)]
    pub fn i2s_mclk_div(&mut self) -> I2sMclkDivW<'_, Cr3Spec> {
        I2sMclkDivW::new(self, 8)
    }
}
#[doc = "control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
