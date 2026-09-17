#[doc = "Register `LLP2` reader"]
pub type R = crate::R<Llp2Spec>;
#[doc = "Register `LLP2` writer"]
pub type W = crate::W<Llp2Spec>;
#[doc = "Field `LOC` reader - The first address of the next LLI chain table"]
pub type LocR = crate::FieldReader<u32>;
#[doc = "Field `LOC` writer - The first address of the next LLI chain table"]
pub type LocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The first address of the next LLI chain table"]
    #[inline(always)]
    pub fn loc(&self) -> LocR {
        LocR::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first address of the next LLI chain table"]
    #[inline(always)]
    pub fn loc(&mut self) -> LocW<'_, Llp2Spec> {
        LocW::new(self, 0)
    }
}
#[doc = "linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp2Spec;
impl crate::RegisterSpec for Llp2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`llp2::R`](R) reader structure"]
impl crate::Readable for Llp2Spec {}
#[doc = "`write(|w| ..)` method takes [`llp2::W`](W) writer structure"]
impl crate::Writable for Llp2Spec {
    type Safety = crate::Unsafe;
}
