#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `OPERATION_DONE` reader - Operation done"]
pub type OperationDoneR = crate::BitReader;
#[doc = "Field `OPERATION_DONE` writer - Operation done"]
pub type OperationDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `READ_NUM_DONE` reader - Read num done"]
pub type ReadNumDoneR = crate::BitReader;
#[doc = "Field `READ_NUM_DONE` writer - Read num done"]
pub type ReadNumDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGRAM_DATA_WAIT` reader - Program data wait"]
pub type ProgramDataWaitR = crate::BitReader;
#[doc = "Field `PROGRAM_DATA_WAIT` writer - Program data wait"]
pub type ProgramDataWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTION_WRITE_ERROR` reader - Option write error"]
pub type OptionWriteErrorR = crate::BitReader;
#[doc = "Field `OPTION_WRITE_ERROR` writer - Option write error"]
pub type OptionWriteErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn operation_done(&self) -> OperationDoneR {
        OperationDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read num done"]
    #[inline(always)]
    pub fn read_num_done(&self) -> ReadNumDoneR {
        ReadNumDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Program data wait"]
    #[inline(always)]
    pub fn program_data_wait(&self) -> ProgramDataWaitR {
        ProgramDataWaitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option write error"]
    #[inline(always)]
    pub fn option_write_error(&self) -> OptionWriteErrorR {
        OptionWriteErrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn operation_done(&mut self) -> OperationDoneW<'_, SrSpec> {
        OperationDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Read num done"]
    #[inline(always)]
    pub fn read_num_done(&mut self) -> ReadNumDoneW<'_, SrSpec> {
        ReadNumDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Program data wait"]
    #[inline(always)]
    pub fn program_data_wait(&mut self) -> ProgramDataWaitW<'_, SrSpec> {
        ProgramDataWaitW::new(self, 2)
    }
    #[doc = "Bit 4 - Option write error"]
    #[inline(always)]
    pub fn option_write_error(&mut self) -> OptionWriteErrorW<'_, SrSpec> {
        OptionWriteErrorW::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
