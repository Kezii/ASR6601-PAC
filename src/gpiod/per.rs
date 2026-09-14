#[doc = "Register `PER` reader"]
pub type R = crate::R<PerSpec>;
#[doc = "Register `PER` writer"]
pub type W = crate::W<PerSpec>;
#[doc = "Field `PE` reader - pin\\[15:0\\] pull-up/pull-down enable"]
pub type PeR = crate::FieldReader<u16>;
#[doc = "Field `PE` writer - pin\\[15:0\\] pull-up/pull-down enable"]
pub type PeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] pull-up/pull-down enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] pull-up/pull-down enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, PerSpec> {
        PeW::new(self, 0)
    }
}
#[doc = "pull enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerSpec;
impl crate::RegisterSpec for PerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per::R`](R) reader structure"]
impl crate::Readable for PerSpec {}
#[doc = "`write(|w| ..)` method takes [`per::W`](W) writer structure"]
impl crate::Writable for PerSpec {
    type Safety = crate::Unsafe;
}
