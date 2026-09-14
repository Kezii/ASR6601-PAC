#[doc = "Register `PPM_ADJUST` reader"]
pub type R = crate::R<PpmAdjustSpec>;
#[doc = "Register `PPM_ADJUST` writer"]
pub type W = crate::W<PpmAdjustSpec>;
#[doc = "Field `PPMADJUST_VALUE` reader - Ppm adjust value"]
pub type PpmadjustValueR = crate::FieldReader<u16>;
#[doc = "Field `PPMADJUST_VALUE` writer - Ppm adjust value"]
pub type PpmadjustValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Ppm adjust value"]
    #[inline(always)]
    pub fn ppmadjust_value(&self) -> PpmadjustValueR {
        PpmadjustValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ppm adjust value"]
    #[inline(always)]
    pub fn ppmadjust_value(&mut self) -> PpmadjustValueW<'_, PpmAdjustSpec> {
        PpmadjustValueW::new(self, 0)
    }
}
#[doc = "ppm adjust value\n\nYou can [`read`](crate::Reg::read) this register and get [`ppm_adjust::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppm_adjust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpmAdjustSpec;
impl crate::RegisterSpec for PpmAdjustSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppm_adjust::R`](R) reader structure"]
impl crate::Readable for PpmAdjustSpec {}
#[doc = "`write(|w| ..)` method takes [`ppm_adjust::W`](W) writer structure"]
impl crate::Writable for PpmAdjustSpec {
    type Safety = crate::Unsafe;
}
