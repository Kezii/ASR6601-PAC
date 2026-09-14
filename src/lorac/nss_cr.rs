#[doc = "Register `NSS_CR` reader"]
pub type R = crate::R<NssCrSpec>;
#[doc = "Register `NSS_CR` writer"]
pub type W = crate::W<NssCrSpec>;
#[doc = "nss control bit"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegNss {
    #[doc = "0: pull down nss pin"]
    PullDown = 0,
    #[doc = "1: pull up nss pin"]
    PullUp = 1,
}
impl From<RegNss> for bool {
    #[inline(always)]
    fn from(variant: RegNss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REG_NSS` reader - nss control bit"]
pub type RegNssR = crate::BitReader<RegNss>;
impl RegNssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RegNss {
        match self.bits {
            false => RegNss::PullDown,
            true => RegNss::PullUp,
        }
    }
    #[doc = "pull down nss pin"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == RegNss::PullDown
    }
    #[doc = "pull up nss pin"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == RegNss::PullUp
    }
}
#[doc = "Field `REG_NSS` writer - nss control bit"]
pub type RegNssW<'a, REG> = crate::BitWriter<'a, REG, RegNss>;
impl<'a, REG> RegNssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pull down nss pin"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(RegNss::PullDown)
    }
    #[doc = "pull up nss pin"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(RegNss::PullUp)
    }
}
impl R {
    #[doc = "Bit 0 - nss control bit"]
    #[inline(always)]
    pub fn reg_nss(&self) -> RegNssR {
        RegNssR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nss control bit"]
    #[inline(always)]
    pub fn reg_nss(&mut self) -> RegNssW<'_, NssCrSpec> {
        RegNssW::new(self, 0)
    }
}
#[doc = "nss control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nss_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nss_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NssCrSpec;
impl crate::RegisterSpec for NssCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nss_cr::R`](R) reader structure"]
impl crate::Readable for NssCrSpec {}
#[doc = "`write(|w| ..)` method takes [`nss_cr::W`](W) writer structure"]
impl crate::Writable for NssCrSpec {
    type Safety = crate::Unsafe;
}
