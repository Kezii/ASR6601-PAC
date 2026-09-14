#[doc = "Register `SCK_CR` reader"]
pub type R = crate::R<SckCrSpec>;
#[doc = "Register `SCK_CR` writer"]
pub type W = crate::W<SckCrSpec>;
#[doc = "sck control bit"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegSck {
    #[doc = "0: pull down sck pin"]
    PullDown = 0,
    #[doc = "1: pull up sck pin"]
    PullUp = 1,
}
impl From<RegSck> for bool {
    #[inline(always)]
    fn from(variant: RegSck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REG_SCK` reader - sck control bit"]
pub type RegSckR = crate::BitReader<RegSck>;
impl RegSckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RegSck {
        match self.bits {
            false => RegSck::PullDown,
            true => RegSck::PullUp,
        }
    }
    #[doc = "pull down sck pin"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == RegSck::PullDown
    }
    #[doc = "pull up sck pin"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == RegSck::PullUp
    }
}
#[doc = "Field `REG_SCK` writer - sck control bit"]
pub type RegSckW<'a, REG> = crate::BitWriter<'a, REG, RegSck>;
impl<'a, REG> RegSckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pull down sck pin"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(RegSck::PullDown)
    }
    #[doc = "pull up sck pin"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(RegSck::PullUp)
    }
}
impl R {
    #[doc = "Bit 0 - sck control bit"]
    #[inline(always)]
    pub fn reg_sck(&self) -> RegSckR {
        RegSckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sck control bit"]
    #[inline(always)]
    pub fn reg_sck(&mut self) -> RegSckW<'_, SckCrSpec> {
        RegSckW::new(self, 0)
    }
}
#[doc = "sck control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sck_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckCrSpec;
impl crate::RegisterSpec for SckCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sck_cr::R`](R) reader structure"]
impl crate::Readable for SckCrSpec {}
#[doc = "`write(|w| ..)` method takes [`sck_cr::W`](W) writer structure"]
impl crate::Writable for SckCrSpec {
    type Safety = crate::Unsafe;
}
