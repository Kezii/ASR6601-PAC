#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MASS_ERASE_EN` reader - Mass erase en"]
pub type MassEraseEnR = crate::BitReader;
#[doc = "Field `MASS_ERASE_EN` writer - Mass erase en"]
pub type MassEraseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_ERASE_EN` reader - Page erase en"]
pub type PageEraseEnR = crate::BitReader;
#[doc = "Field `PAGE_ERASE_EN` writer - Page erase en"]
pub type PageEraseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_EN` reader - Prog en"]
pub type ProgEnR = crate::BitReader;
#[doc = "Field `PROG_EN` writer - Prog en"]
pub type ProgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "flash program mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgMode {
    #[doc = "0: single programming mode"]
    Single = 0,
    #[doc = "1: word line continuous programming mode"]
    WlContinuous = 1,
}
impl From<ProgMode> for bool {
    #[inline(always)]
    fn from(variant: ProgMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG_MODE` reader - flash program mode selection"]
pub type ProgModeR = crate::BitReader<ProgMode>;
impl ProgModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProgMode {
        match self.bits {
            false => ProgMode::Single,
            true => ProgMode::WlContinuous,
        }
    }
    #[doc = "single programming mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ProgMode::Single
    }
    #[doc = "word line continuous programming mode"]
    #[inline(always)]
    pub fn is_wl_continuous(&self) -> bool {
        *self == ProgMode::WlContinuous
    }
}
#[doc = "Field `PROG_MODE` writer - flash program mode selection"]
pub type ProgModeW<'a, REG> = crate::BitWriter<'a, REG, ProgMode>;
impl<'a, REG> ProgModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "single programming mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(ProgMode::Single)
    }
    #[doc = "word line continuous programming mode"]
    #[inline(always)]
    pub fn wl_continuous(self) -> &'a mut crate::W<REG> {
        self.variant(ProgMode::WlContinuous)
    }
}
#[doc = "Field `READ_ACC_EN` reader - Read acc en"]
pub type ReadAccEnR = crate::BitReader;
#[doc = "Field `READ_ACC_EN` writer - Read acc en"]
pub type ReadAccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH_EN` reader - Prefetch en"]
pub type PrefetchEnR = crate::BitReader;
#[doc = "Field `PREFETCH_EN` writer - Prefetch en"]
pub type PrefetchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_RELEASE_EN` reader - Write release en"]
pub type WriteReleaseEnR = crate::BitReader;
#[doc = "Field `WRITE_RELEASE_EN` writer - Write release en"]
pub type WriteReleaseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTION_OPR_EN` reader - Option operation enable"]
pub type OptionOprEnR = crate::BitReader;
#[doc = "Field `OPTION_OPR_EN` writer - Option operation enable"]
pub type OptionOprEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_DIS` reader - ECC encoding disable"]
pub type EccDisR = crate::BitReader;
#[doc = "Field `ECC_DIS` writer - ECC encoding disable"]
pub type EccDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFO_BYTE_LOAD` reader - Info byte load reset request"]
pub type InfoByteLoadR = crate::BitReader;
#[doc = "Field `INFO_BYTE_LOAD` writer - Info byte load reset request"]
pub type InfoByteLoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mass erase en"]
    #[inline(always)]
    pub fn mass_erase_en(&self) -> MassEraseEnR {
        MassEraseEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase en"]
    #[inline(always)]
    pub fn page_erase_en(&self) -> PageEraseEnR {
        PageEraseEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Prog en"]
    #[inline(always)]
    pub fn prog_en(&self) -> ProgEnR {
        ProgEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - flash program mode selection"]
    #[inline(always)]
    pub fn prog_mode(&self) -> ProgModeR {
        ProgModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read acc en"]
    #[inline(always)]
    pub fn read_acc_en(&self) -> ReadAccEnR {
        ReadAccEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch en"]
    #[inline(always)]
    pub fn prefetch_en(&self) -> PrefetchEnR {
        PrefetchEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write release en"]
    #[inline(always)]
    pub fn write_release_en(&self) -> WriteReleaseEnR {
        WriteReleaseEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Option operation enable"]
    #[inline(always)]
    pub fn option_opr_en(&self) -> OptionOprEnR {
        OptionOprEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ECC encoding disable"]
    #[inline(always)]
    pub fn ecc_dis(&self) -> EccDisR {
        EccDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Info byte load reset request"]
    #[inline(always)]
    pub fn info_byte_load(&self) -> InfoByteLoadR {
        InfoByteLoadR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mass erase en"]
    #[inline(always)]
    pub fn mass_erase_en(&mut self) -> MassEraseEnW<'_, CrSpec> {
        MassEraseEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase en"]
    #[inline(always)]
    pub fn page_erase_en(&mut self) -> PageEraseEnW<'_, CrSpec> {
        PageEraseEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Prog en"]
    #[inline(always)]
    pub fn prog_en(&mut self) -> ProgEnW<'_, CrSpec> {
        ProgEnW::new(self, 2)
    }
    #[doc = "Bit 3 - flash program mode selection"]
    #[inline(always)]
    pub fn prog_mode(&mut self) -> ProgModeW<'_, CrSpec> {
        ProgModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Read acc en"]
    #[inline(always)]
    pub fn read_acc_en(&mut self) -> ReadAccEnW<'_, CrSpec> {
        ReadAccEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Prefetch en"]
    #[inline(always)]
    pub fn prefetch_en(&mut self) -> PrefetchEnW<'_, CrSpec> {
        PrefetchEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Write release en"]
    #[inline(always)]
    pub fn write_release_en(&mut self) -> WriteReleaseEnW<'_, CrSpec> {
        WriteReleaseEnW::new(self, 6)
    }
    #[doc = "Bit 8 - Option operation enable"]
    #[inline(always)]
    pub fn option_opr_en(&mut self) -> OptionOprEnW<'_, CrSpec> {
        OptionOprEnW::new(self, 8)
    }
    #[doc = "Bit 9 - ECC encoding disable"]
    #[inline(always)]
    pub fn ecc_dis(&mut self) -> EccDisW<'_, CrSpec> {
        EccDisW::new(self, 9)
    }
    #[doc = "Bit 31 - Info byte load reset request"]
    #[inline(always)]
    pub fn info_byte_load(&mut self) -> InfoByteLoadW<'_, CrSpec> {
        InfoByteLoadW::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
